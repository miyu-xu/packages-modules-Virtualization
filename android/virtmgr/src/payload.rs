// Copyright 2021, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Payload disk image

use crate::debug_config::DebugConfig;
use android_system_virtualizationservice::aidl::android::system::virtualizationservice::{
    DiskImage::DiskImage,
    Partition::Partition,
    VirtualMachineAppConfig::DebugLevel::DebugLevel,
    VirtualMachineAppConfig::{Payload::Payload, VirtualMachineAppConfig},
    VirtualMachineRawConfig::VirtualMachineRawConfig,
};
use anyhow::{anyhow, bail, Context, Result};
#[cfg(target_os = "android")]
use binder::wait_for_interface;
use binder::ParcelFileDescriptor;
use log::{info, warn};
use microdroid_metadata::{ApexPayload, ApkPayload, Metadata, PayloadConfig, PayloadMetadata};
use microdroid_payload_config::{ApexConfig, VmPayloadConfig};
#[cfg(target_os = "android")]
use once_cell::sync::OnceCell;
use packagemanager_aidl::aidl::android::content::pm::{
    IPackageManagerNative::IPackageManagerNative, StagedApexInfo,
};
use regex::Regex;
use serde::Deserialize;
use serde_xml_rs::from_reader;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::{metadata, File, OpenOptions};
#[cfg(not(target_os = "android"))]
use std::io::{copy, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;
use vmconfig::{open_parcel_file, resolve_host_path};
#[cfg(not(target_os = "android"))]
use zip::ZipArchive;

const APEX_INFO_LIST_PATH: &str = "/apex/apex-info-list.xml";

const PACKAGE_MANAGER_NATIVE_SERVICE: &str = "package_native";
#[cfg(not(target_os = "android"))]
const WINDOWS_STAGED_APEX_DIR_ENV: &str = "VIRTMGR_STAGED_APEX_DIR";
#[cfg(not(target_os = "android"))]
const WINDOWS_STAGED_APEX_JSON: &str = "staged_apexes.json";
#[cfg(not(target_os = "android"))]
const WINDOWS_STAGED_APEX_STATE_JSON: &str = "staged_state.json";
#[cfg(not(target_os = "android"))]
const WINDOWS_STAGED_DECOMPRESSED_DIR: &str = "decompressed";
/// Optional JSON array (same shape as `staged_apexes.json`) to **mock** `IPackageManagerNative`
/// staged APEX metadata without a directory scan (merged with `VIRTMGR_STAGED_APEX_DIR` when both set).
#[cfg(not(target_os = "android"))]
const WINDOWS_MOCK_STAGED_APEX_JSON_ENV: &str = "VIRTMGR_MOCK_STAGED_APEX_JSON";

/// Represents the list of APEXes
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct ApexInfoList {
    #[serde(rename = "apex-info")]
    list: Vec<ApexInfo>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
struct ApexInfo {
    #[serde(rename = "moduleName")]
    name: String,
    #[serde(rename = "versionCode")]
    version: u64,
    #[serde(rename = "modulePath")]
    path: PathBuf,

    #[serde(default)]
    has_classpath_jar: bool,

    // The field claims to be milliseconds but is actually seconds.
    #[serde(rename = "lastUpdateMillis")]
    last_update_seconds: u64,

    #[serde(rename = "isFactory")]
    is_factory: bool,

    #[serde(rename = "isActive")]
    is_active: bool,

    #[serde(rename = "provideSharedApexLibs")]
    provide_shared_apex_libs: bool,

    #[serde(rename = "preinstalledModulePath")]
    preinstalled_path: PathBuf,
}

impl ApexInfoList {
    /// Loads ApexInfoList
    fn load(temporary_directory: &Path) -> Result<ApexInfoList> {
        #[cfg(target_os = "android")]
        {
            static INSTANCE: OnceCell<ApexInfoList> = OnceCell::new();
            return INSTANCE.get_or_try_init(Self::load_from_xml).cloned();
        }
        #[cfg(not(target_os = "android"))]
        {
            let apex_info_list_path = resolve_host_path(Path::new(APEX_INFO_LIST_PATH));
            if !apex_info_list_path.exists() {
                return load_desktop_factory_apex_info_list(temporary_directory);
            }
            Self::load_from_xml()
        }
    }

    fn load_from_xml() -> Result<ApexInfoList> {
        let apex_info_list_path = resolve_host_path(Path::new(APEX_INFO_LIST_PATH));
        let apex_info_list = File::open(&apex_info_list_path)
            .with_context(|| format!("Failed to open {}", apex_info_list_path.display()))?;
        let mut apex_info_list: ApexInfoList = from_reader(apex_info_list)
            .with_context(|| format!("Failed to parse {}", apex_info_list_path.display()))?;

        // For active APEXes, we run derive_classpath and parse its output to see if it
        // contributes to the classpath(s). (This allows us to handle any new classpath env
        // vars seamlessly.)
        if cfg!(target_os = "android") && !cfg!(early) {
            let classpath_vars = run_derive_classpath()?;
            let classpath_apexes = find_apex_names_in_classpath(&classpath_vars)?;

            for apex_info in apex_info_list.list.iter_mut() {
                apex_info.has_classpath_jar = classpath_apexes.contains(&apex_info.name);
            }
        }

        Ok(apex_info_list)
    }

    // Override apex info with the staged one
    fn override_staged_apex(&mut self, staged_apex_info: &StagedApexInfo) -> Result<()> {
        let mut need_to_add: Option<ApexInfo> = None;
        for apex_info in self.list.iter_mut() {
            if staged_apex_info.moduleName == apex_info.name {
                if apex_info.is_active && apex_info.is_factory {
                    // Copy the entry to the end as factory/non-active after the loop
                    // to keep the factory version. Typically this step is unncessary,
                    // but some apexes (like sharedlibs) need to be kept even if it's inactive.
                    need_to_add.replace(ApexInfo { is_active: false, ..apex_info.clone() });
                    // And make this one as non-factory. Note that this one is still active
                    // and overridden right below.
                    apex_info.is_factory = false;
                }
                // Active one is overridden with the staged one.
                if apex_info.is_active {
                    apex_info.version = staged_apex_info.versionCode as u64;
                    apex_info.path = PathBuf::from(&staged_apex_info.diskImagePath);
                    apex_info.has_classpath_jar = staged_apex_info.hasClassPathJars;
                    apex_info.last_update_seconds = last_updated(&apex_info.path)?;
                }
            }
        }
        if let Some(info) = need_to_add {
            self.list.push(info);
        }
        Ok(())
    }
}

fn last_updated<P: AsRef<Path>>(path: P) -> Result<u64> {
    let metadata = metadata(path)?;
    Ok(metadata.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}

impl ApexInfo {
    fn matches(&self, apex_config: &ApexConfig) -> bool {
        // Match with pseudo name "{CLASSPATH}" which represents APEXes contributing
        // to any derive_classpath environment variable
        if apex_config.name == "{CLASSPATH}" && self.has_classpath_jar {
            return true;
        }
        if apex_config.name == self.name {
            return true;
        }
        false
    }
}

struct PackageManager {
    apex_info_list: ApexInfoList,
}

#[cfg(not(target_os = "android"))]
#[derive(Debug, Deserialize)]
struct WindowsStagedApexEntry {
    module_name: String,
    disk_image_path: String,
    #[serde(default)]
    version_code: i64,
    #[serde(default)]
    has_classpath_jars: bool,
}

#[cfg(not(target_os = "android"))]
#[derive(Debug, Default, Deserialize)]
struct WindowsStagedState {
    #[serde(default)]
    active_modules: Vec<String>,
}

#[cfg(not(target_os = "android"))]
fn load_windows_staged_state(staged_root: &Path) -> Result<Option<HashSet<String>>> {
    let state_path = staged_root.join(WINDOWS_STAGED_APEX_STATE_JSON);
    if !state_path.exists() {
        return Ok(None);
    }
    let file = File::open(&state_path)
        .with_context(|| format!("Failed to open {}", state_path.display()))?;
    let state: WindowsStagedState = serde_json::from_reader(file)
        .with_context(|| format!("Failed to parse {}", state_path.display()))?;
    let active: HashSet<String> =
        state.active_modules.into_iter().filter(|m| !m.is_empty()).collect();
    Ok(Some(active))
}

#[cfg(not(target_os = "android"))]
fn load_mock_staged_apex_json() -> Result<Vec<StagedApexInfo>> {
    let path = match std::env::var(WINDOWS_MOCK_STAGED_APEX_JSON_ENV) {
        Ok(p) if !p.is_empty() => PathBuf::from(p),
        _ => return Ok(vec![]),
    };
    let file = File::open(&path).with_context(|| format!("Failed to open {}", path.display()))?;
    let entries: Vec<WindowsStagedApexEntry> = serde_json::from_reader(file)
        .with_context(|| format!("Failed to parse {}", path.display()))?;
    Ok(entries
        .into_iter()
        .map(|e| StagedApexInfo {
            moduleName: e.module_name,
            diskImagePath: e.disk_image_path,
            versionCode: e.version_code,
            hasClassPathJars: e.has_classpath_jars,
            ..Default::default()
        })
        .collect())
}

#[cfg(not(target_os = "android"))]
fn extract_original_apex_if_needed(capex_path: &Path, output_path: &Path) -> Result<()> {
    let refresh = match (metadata(capex_path), metadata(output_path)) {
        (Ok(source), Ok(target)) => source.modified()? > target.modified()?,
        (Ok(_), Err(_)) => true,
        (Err(err), _) => return Err(err.into()),
    };
    if !refresh {
        return Ok(());
    }

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    let file = File::open(capex_path)
        .with_context(|| format!("Failed to open {}", capex_path.display()))?;
    let mut archive = ZipArchive::new(file)
        .with_context(|| format!("Failed to read {}", capex_path.display()))?;
    let mut entry = archive.by_name("original_apex").with_context(|| {
        format!("CAPEX does not contain original_apex: {}", capex_path.display())
    })?;
    let mut output = File::create(output_path)
        .with_context(|| format!("Failed to create {}", output_path.display()))?;
    copy(&mut entry, &mut output)
        .with_context(|| format!("Failed to extract {}", output_path.display()))?;
    Ok(())
}

#[cfg(not(target_os = "android"))]
fn resolve_windows_staged_disk_image_path(
    staged_root: &Path,
    disk_image_path: &str,
) -> Result<PathBuf> {
    let mut path = PathBuf::from(disk_image_path);
    if path.is_relative() {
        path = staged_root.join(path);
    }
    if path.extension().and_then(OsStr::to_str) == Some("capex") {
        let module_name = path
            .file_stem()
            .and_then(OsStr::to_str)
            .ok_or_else(|| anyhow!("Invalid staged CAPEX path {}", path.display()))?;
        let decompressed =
            staged_root.join(WINDOWS_STAGED_DECOMPRESSED_DIR).join(format!("{module_name}.apex"));
        extract_original_apex_if_needed(&path, &decompressed)?;
        return Ok(decompressed);
    }
    Ok(path)
}

#[cfg(not(target_os = "android"))]
fn load_desktop_factory_apex_info_list(temporary_directory: &Path) -> Result<ApexInfoList> {
    let roots = [
        (resolve_host_path(Path::new("/system/apex")), Path::new("/system/apex")),
        (resolve_host_path(Path::new("/system_ext/apex")), Path::new("/system_ext/apex")),
    ];
    let cache = temporary_directory.join("factory-apexes");
    std::fs::create_dir_all(&cache)
        .with_context(|| format!("Failed to create {}", cache.display()))?;
    let mut seen = HashSet::new();
    let mut list = Vec::new();

    for (root, logical_root) in roots {
        if !root.is_dir() {
            continue;
        }
        let mut entries = std::fs::read_dir(&root)
            .with_context(|| format!("Failed to read {}", root.display()))?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            if !entry.file_type()?.is_file() {
                continue;
            }
            let source = entry.path();
            let extension = source.extension().and_then(OsStr::to_str);
            if extension != Some("apex") && extension != Some("capex") {
                continue;
            }
            let module = source
                .file_stem()
                .and_then(OsStr::to_str)
                .ok_or_else(|| anyhow!("Invalid factory APEX path {}", source.display()))?;
            if module.is_empty()
                || !module
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
            {
                bail!("Invalid factory APEX module name in {}", source.display());
            }
            if !seen.insert(module.to_owned()) {
                bail!("Duplicate active factory APEX module {module}");
            }
            let path = if extension == Some("capex") {
                let output = cache.join(format!("{module}.apex"));
                extract_original_apex_if_needed(&source, &output)?;
                output
            } else {
                source.clone()
            };
            list.push(ApexInfo {
                name: module.to_owned(),
                version: 0,
                path: path.clone(),
                has_classpath_jar: false,
                // CAPEX files are expanded into a per-VM temporary directory.  The expanded
                // file therefore has a new timestamp on every launch, but
                // `last_update_seconds` is part of Microdroid's persisted payload identity.
                // Use the immutable product artifact timestamp so an unchanged product can
                // restart the same instance without being rejected as a changed APEX set.
                last_update_seconds: last_updated(&source)?,
                is_factory: true,
                is_active: true,
                provide_shared_apex_libs: false,
                preinstalled_path: logical_root.join(entry.file_name()),
            });
        }
    }

    if list.is_empty() {
        bail!("Desktop Microdroid product contains no factory APEX files");
    }
    // Directory enumeration order is not a cross-platform contract.  Partition numbering is
    // derived from this list, so keep the final order stable even if roots or scan code change.
    list.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.preinstalled_path.cmp(&right.preinstalled_path))
    });
    Ok(ApexInfoList { list })
}

#[cfg(not(target_os = "android"))]
fn collect_windows_staged_apexes() -> Result<Vec<StagedApexInfo>> {
    let mut out = load_mock_staged_apex_json()?;

    let staged_root = match std::env::var(WINDOWS_STAGED_APEX_DIR_ENV) {
        Ok(s) if !s.is_empty() => PathBuf::from(s),
        _ => {
            if out.is_empty() {
                bail!(
                    "prefer_staged on desktop host requires VIRTMGR_STAGED_APEX_DIR \
                     and/or VIRTMGR_MOCK_STAGED_APEX_JSON"
                );
            }
            return Ok(out);
        }
    };

    if !staged_root.is_dir() {
        bail!("VIRTMGR_STAGED_APEX_DIR is not a directory: {}", staged_root.display());
    }
    let active_modules = load_windows_staged_state(&staged_root)?;

    let json_path = staged_root.join(WINDOWS_STAGED_APEX_JSON);
    if json_path.exists() {
        let file = File::open(&json_path)
            .with_context(|| format!("Failed to open {}", json_path.display()))?;
        let mut entries: Vec<WindowsStagedApexEntry> = serde_json::from_reader(file)
            .with_context(|| format!("Failed to parse {}", json_path.display()))?;
        if let Some(active) = &active_modules {
            entries.retain(|e| active.contains(&e.module_name));
        }
        for entry in entries {
            let disk_image_path =
                resolve_windows_staged_disk_image_path(&staged_root, &entry.disk_image_path)?;
            out.push(StagedApexInfo {
                moduleName: entry.module_name,
                diskImagePath: disk_image_path.to_string_lossy().to_string(),
                versionCode: entry.version_code,
                hasClassPathJars: entry.has_classpath_jars,
                ..Default::default()
            });
        }
        return Ok(out);
    }

    for entry in std::fs::read_dir(&staged_root)
        .with_context(|| format!("Failed to read {}", staged_root.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().and_then(OsStr::to_str);
        if extension != Some("apex") && extension != Some("capex") {
            continue;
        }
        let module = match path.file_stem().and_then(OsStr::to_str) {
            Some(m) if !m.is_empty() => m.to_string(),
            _ => continue,
        };
        if let Some(active) = &active_modules {
            if !active.contains(&module) {
                continue;
            }
        }
        out.push(StagedApexInfo {
            moduleName: module,
            diskImagePath: resolve_windows_staged_disk_image_path(
                &staged_root,
                &path.to_string_lossy(),
            )
            .map(|p| p.to_string_lossy().to_string())?,
            versionCode: 0,
            hasClassPathJars: false,
            ..Default::default()
        });
    }
    Ok(out)
}

impl PackageManager {
    fn new(temporary_directory: &Path) -> Result<Self> {
        let apex_info_list = ApexInfoList::load(temporary_directory)?;
        Ok(Self { apex_info_list })
    }

    fn get_apex_list(&self, prefer_staged: bool) -> Result<ApexInfoList> {
        // get the list of active apexes
        let mut list = self.apex_info_list.clone();
        // When prefer_staged, we override ApexInfo by consulting "package_native"
        if prefer_staged {
            #[cfg(not(target_os = "android"))]
            {
                let staged = collect_windows_staged_apexes()?;
                for staged_apex_info in staged {
                    list.override_staged_apex(&staged_apex_info)?;
                }
            }
            #[cfg(target_os = "android")]
            {
                if cfg!(early) {
                    return Err(anyhow!("Can't turn on prefer_staged on early boot VMs"));
                }
                let pm =
                    wait_for_interface::<dyn IPackageManagerNative>(PACKAGE_MANAGER_NATIVE_SERVICE)
                        .context("Failed to get service when prefer_staged is set.")?;
                let staged =
                    pm.getStagedApexModuleNames().context("getStagedApexModuleNames failed")?;
                for name in staged {
                    if let Some(staged_apex_info) =
                        pm.getStagedApexInfo(&name).context("getStagedApexInfo failed")?
                    {
                        list.override_staged_apex(&staged_apex_info)?;
                    }
                }
            }
        }
        Ok(list)
    }
}

fn make_metadata_file(
    app_config: &VirtualMachineAppConfig,
    apex_infos: &[&ApexInfo],
    temporary_directory: &Path,
) -> Result<ParcelFileDescriptor> {
    let payload_metadata = match &app_config.payload {
        Payload::PayloadConfig(payload_config) => PayloadMetadata::Config(PayloadConfig {
            payload_binary_name: payload_config.payloadBinaryName.clone(),
            extra_apk_count: payload_config.extraApks.len().try_into()?,
            special_fields: Default::default(),
        }),
        Payload::ConfigPath(config_path) => {
            PayloadMetadata::ConfigPath(format!("/mnt/apk/{}", config_path))
        }
    };

    let metadata = Metadata {
        version: 1,
        apexes: apex_infos
            .iter()
            .enumerate()
            .map(|(i, apex_info)| {
                Ok(ApexPayload {
                    name: apex_info.name.clone(),
                    partition_name: format!("microdroid-apex-{}", i),
                    last_update_seconds: apex_info.last_update_seconds,
                    is_factory: apex_info.is_factory,
                    ..Default::default()
                })
            })
            .collect::<Result<_>>()?,
        apk: Some(ApkPayload {
            name: "apk".to_owned(),
            payload_partition_name: "microdroid-apk".to_owned(),
            idsig_partition_name: "microdroid-apk-idsig".to_owned(),
            ..Default::default()
        })
        .into(),
        payload: Some(payload_metadata),
        ..Default::default()
    };

    // Write metadata to file.
    let metadata_path = temporary_directory.join("metadata");
    let mut metadata_file = OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(&metadata_path)
        .with_context(|| format!("Failed to open metadata file {:?}", metadata_path))?;
    microdroid_metadata::write_metadata(&metadata, &mut metadata_file)?;

    // Re-open the metadata file as read-only.
    open_parcel_file(&metadata_path, false)
}

/// Creates a DiskImage with partitions:
///   payload-metadata: metadata
///   microdroid-apex-0: apex 0
///   microdroid-apex-1: apex 1
///   ..
///   microdroid-apk: apk
///   microdroid-apk-idsig: idsig
///   extra-apk-0:   additional apk 0
///   extra-idsig-0: additional idsig 0
///   extra-apk-1:   additional apk 1
///   extra-idsig-1: additional idsig 1
///   ..
fn make_payload_disk(
    app_config: &VirtualMachineAppConfig,
    debug_config: &DebugConfig,
    apk_file: File,
    idsig_file: File,
    extra_apk_files: Vec<File>,
    vm_payload_config: &VmPayloadConfig,
    temporary_directory: &Path,
) -> Result<DiskImage> {
    if extra_apk_files.len() != app_config.extraIdsigs.len() {
        bail!(
            "payload config has {} apks, but app config has {} idsigs",
            vm_payload_config.extra_apks.len(),
            app_config.extraIdsigs.len()
        );
    }

    let pm = PackageManager::new(temporary_directory)?;
    let apex_list = pm.get_apex_list(vm_payload_config.prefer_staged)?;

    // collect APEXes from config
    let mut apex_infos = collect_apex_infos(&apex_list, &vm_payload_config.apexes, debug_config)?;

    // Pass sorted list of apexes. Sorting key shouldn't use `path` because it will change after
    // reboot with prefer_staged. `last_update_seconds` is added to distinguish "samegrade"
    // update.
    apex_infos.sort_by_key(|info| (&info.name, &info.version, &info.last_update_seconds));
    info!("Microdroid payload APEXes: {:?}", apex_infos.iter().map(|ai| &ai.name));

    let metadata_file = make_metadata_file(app_config, &apex_infos, temporary_directory)?;
    // put metadata at the first partition
    let mut partitions = vec![Partition {
        label: "payload-metadata".to_owned(),
        image: Some(metadata_file),
        writable: false,
        guid: None,
    }];

    for (i, apex_info) in apex_infos.iter().enumerate() {
        let path = if cfg!(early) {
            let path = &apex_info.preinstalled_path;
            if path.extension().and_then(OsStr::to_str).unwrap_or("") != "apex" {
                bail!("compressed APEX {} not supported", path.display());
            }
            path
        } else {
            &apex_info.path
        };
        let apex_file = open_parcel_file(path, false)?;
        partitions.push(Partition {
            label: format!("microdroid-apex-{}", i),
            image: Some(apex_file),
            writable: false,
            guid: None,
        });
    }
    partitions.push(Partition {
        label: "microdroid-apk".to_owned(),
        image: Some(ParcelFileDescriptor::new(apk_file)),
        writable: false,
        guid: None,
    });
    partitions.push(Partition {
        label: "microdroid-apk-idsig".to_owned(),
        image: Some(ParcelFileDescriptor::new(idsig_file)),
        writable: false,
        guid: None,
    });

    // we've already checked that extra_apks and extraIdsigs are in the same size.
    let extra_idsigs = &app_config.extraIdsigs;
    for (i, (extra_apk_file, extra_idsig)) in
        extra_apk_files.into_iter().zip(extra_idsigs.iter()).enumerate()
    {
        partitions.push(Partition {
            label: format!("extra-apk-{i}"),
            image: Some(ParcelFileDescriptor::new(extra_apk_file)),
            writable: false,
            guid: None,
        });

        partitions.push(Partition {
            label: format!("extra-idsig-{i}"),
            image: Some(ParcelFileDescriptor::new(
                extra_idsig
                    .as_ref()
                    .try_clone()
                    .with_context(|| format!("Failed to clone the extra idsig #{i}"))?,
            )),
            writable: false,
            guid: None,
        });
    }

    Ok(DiskImage { image: None, partitions, writable: false })
}

fn run_derive_classpath() -> Result<String> {
    #[cfg(not(target_os = "android"))]
    {
        return Ok(String::new());
    }
    let result = Command::new("/apex/com.android.sdkext/bin/derive_classpath")
        .arg("/proc/self/fd/1")
        .output()
        .context("Failed to run derive_classpath")?;

    if !result.status.success() {
        bail!("derive_classpath returned {}", result.status);
    }

    String::from_utf8(result.stdout).context("Converting derive_classpath output")
}

fn find_apex_names_in_classpath(classpath_vars: &str) -> Result<HashSet<String>> {
    // Each line should be in the format "export <var name> <paths>", where <paths> is a
    // colon-separated list of paths to JARs. We don't care about the var names, and we're only
    // interested in paths that look like "/apex/<apex name>/<anything>" so we know which APEXes
    // contribute to at least one var.
    let mut apexes = HashSet::new();

    let pattern = Regex::new(r"^export [^ ]+ ([^ ]+)$").context("Failed to construct Regex")?;
    for line in classpath_vars.lines() {
        if let Some(captures) = pattern.captures(line) {
            if let Some(paths) = captures.get(1) {
                apexes.extend(paths.as_str().split(':').filter_map(|path| {
                    let path = path.strip_prefix("/apex/")?;
                    Some(path[..path.find('/')?].to_owned())
                }));
                continue;
            }
        }
        warn!("Malformed line from derive_classpath: {}", line);
    }

    Ok(apexes)
}

fn check_apexes_are_from_allowed_partitions(requested_apexes: &Vec<&ApexInfo>) -> Result<()> {
    const ALLOWED_PARTITIONS: [&str; 2] = ["/system", "/system_ext"];
    for apex in requested_apexes {
        if !ALLOWED_PARTITIONS.iter().any(|p| apex.preinstalled_path.starts_with(p)) {
            bail!("Non-system APEX {} is not supported in Microdroid", apex.name);
        }
    }
    Ok(())
}

// Collect ApexInfos from VM config
fn collect_apex_infos<'a>(
    apex_list: &'a ApexInfoList,
    apex_configs: &[ApexConfig],
    debug_config: &DebugConfig,
) -> Result<Vec<&'a ApexInfo>> {
    // APEXes which any Microdroid VM needs.
    // TODO(b/192200378) move this to microdroid.json?
    let required_apexes: &[_] =
        if debug_config.should_include_debug_apexes() { &["com.android.adbd"] } else { &[] };

    let apex_infos = apex_list
        .list
        .iter()
        .filter(|ai| {
            apex_configs.iter().any(|cfg| ai.matches(cfg) && ai.is_active)
                || required_apexes.iter().any(|name| name == &ai.name && ai.is_active)
                || ai.provide_shared_apex_libs
        })
        .collect();

    check_apexes_are_from_allowed_partitions(&apex_infos)?;
    Ok(apex_infos)
}

pub fn add_microdroid_vendor_image(vendor_image: File, vm_config: &mut VirtualMachineRawConfig) {
    vm_config.disks.push(DiskImage {
        image: None,
        writable: false,
        partitions: vec![Partition {
            label: "microdroid-vendor".to_owned(),
            image: Some(ParcelFileDescriptor::new(vendor_image)),
            writable: false,
            guid: None,
        }],
    })
}

pub fn add_microdroid_system_images(
    config: &VirtualMachineAppConfig,
    instance_file: File,
    storage_image: Option<File>,
    os_name: &str,
    vm_config: &mut VirtualMachineRawConfig,
) -> Result<()> {
    #[cfg(not(target_os = "android"))]
    fn ensure_desktop_microdroid_boot_param(
        vm_config: &mut VirtualMachineRawConfig,
        key: &str,
        value: &str,
    ) {
        let param = format!("{key}={value}");
        if vm_config
            .params
            .as_deref()
            .is_some_and(|params| params.split_whitespace().any(|existing| existing == param))
        {
            return;
        }
        match vm_config.params.as_mut() {
            Some(params) if !params.is_empty() => {
                params.push(' ');
                params.push_str(&param);
            }
            Some(params) => params.push_str(&param),
            None => vm_config.params = Some(param),
        }
    }

    #[cfg(not(target_os = "android"))]
    fn ensure_desktop_microdroid_boot_params_from_initrd(
        vm_config: &mut VirtualMachineRawConfig,
        initrd_path: &str,
        keys: &[&str],
    ) -> Result<()> {
        const BOOTCONFIG_MAGIC: &[u8] = b"#BOOTCONFIG\n";
        const INITRD_FOOTER_LEN: u64 =
            (2 * std::mem::size_of::<u32>() + BOOTCONFIG_MAGIC.len()) as u64;

        let mut initrd = File::open(initrd_path)
            .with_context(|| format!("Failed to open initrd bootconfig source {}", initrd_path))?;
        let initrd_len = initrd.metadata()?.len();
        if initrd_len < INITRD_FOOTER_LEN {
            bail!("Initrd {} is too small to contain bootconfig footer", initrd_path);
        }

        let tail_len = initrd_len.min(1024 * 1024) as usize;
        let tail_offset = initrd_len - tail_len as u64;
        let mut tail = vec![0u8; tail_len];
        initrd.seek(SeekFrom::Start(tail_offset))?;
        initrd.read_exact(&mut tail)?;
        let Some(magic_pos) =
            tail.windows(BOOTCONFIG_MAGIC.len()).rposition(|window| window == BOOTCONFIG_MAGIC)
        else {
            bail!("Initrd {} is missing the bootconfig footer", initrd_path);
        };
        let footer_offset =
            tail_offset + magic_pos as u64 + BOOTCONFIG_MAGIC.len() as u64 - INITRD_FOOTER_LEN;
        if footer_offset + INITRD_FOOTER_LEN > initrd_len {
            bail!("Initrd {} has an invalid bootconfig footer offset", initrd_path);
        }

        let mut size_bytes = [0u8; std::mem::size_of::<u32>()];
        initrd.seek(SeekFrom::Start(footer_offset))?;
        initrd.read_exact(&mut size_bytes)?;
        let bootconfig_len = u32::from_le_bytes(size_bytes) as u64;
        if bootconfig_len > footer_offset {
            bail!(
                "Initrd {} has an invalid bootconfig footer size {}",
                initrd_path,
                bootconfig_len
            );
        }

        let bootconfig_offset = footer_offset - bootconfig_len;
        let mut bootconfig = vec![0u8; bootconfig_len as usize];
        initrd.seek(SeekFrom::Start(bootconfig_offset))?;
        initrd.read_exact(&mut bootconfig)?;

        for line in String::from_utf8_lossy(&bootconfig).lines() {
            let line = line.trim_matches(char::from(0)).trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let key = key.trim();
            if !keys.iter().any(|wanted| *wanted == key) {
                continue;
            }

            let value = value.trim().trim_matches('"');
            if value.is_empty() {
                continue;
            }
            ensure_desktop_microdroid_boot_param(vm_config, key, value);
        }

        Ok(())
    }

    let debug_suffix = match config.debugLevel {
        DebugLevel::NONE => "normal",
        DebugLevel::FULL => "debuggable",
        _ => return Err(anyhow!("unsupported debug level: {:?}", config.debugLevel)),
    };
    let initrd = format!("/apex/com.android.virt/etc/{os_name}_initrd_{debug_suffix}.img");
    vm_config.initrd = Some(open_parcel_file(Path::new(&initrd), false)?);

    #[cfg(not(target_os = "android"))]
    if os_name == "microdroid" || os_name.starts_with("microdroid_gki-") {
        #[cfg(target_arch = "aarch64")]
        let initrd_host_path = resolve_host_path(Path::new(&initrd));
        #[cfg(target_arch = "aarch64")]
        ensure_desktop_microdroid_boot_params_from_initrd(
            vm_config,
            initrd_host_path
                .to_str()
                .ok_or_else(|| anyhow!("Invalid initrd path {}", initrd_host_path.display()))?,
            &[
                "androidboot.vbmeta.size",
                "androidboot.vbmeta.digest",
                "androidboot.vbmeta.hash_alg",
                "androidboot.vbmeta.avb_version",
                "androidboot.vbmeta.invalidate_on_error",
                "androidboot.vbmeta.device_state",
                "androidboot.vbmeta.device",
            ],
        )?;
        #[cfg(target_arch = "aarch64")]
        ensure_desktop_microdroid_boot_param(vm_config, "androidboot.boot_devices", "10000.pci");
        ensure_desktop_microdroid_boot_param(vm_config, "androidboot.slot_suffix", "_a");
        ensure_desktop_microdroid_boot_param(
            vm_config,
            "androidboot.vbmeta.device",
            "/dev/block/by-name/vbmeta_a",
        );
        ensure_desktop_microdroid_boot_param(
            vm_config,
            "androidboot.vbmeta.device_state",
            "locked",
        );
    }

    let mut writable_partitions = vec![Partition {
        label: "vm-instance".to_owned(),
        image: Some(ParcelFileDescriptor::new(instance_file)),
        writable: true,
        guid: None,
    }];

    if let Some(file) = storage_image {
        writable_partitions.push(Partition {
            label: "encryptedstore".to_owned(),
            image: Some(ParcelFileDescriptor::new(file)),
            writable: true,
            guid: None,
        });
    }

    vm_config.disks.push(DiskImage {
        image: None,
        partitions: writable_partitions,
        writable: true,
    });

    Ok(())
}

#[allow(clippy::too_many_arguments)] // TODO: Fewer arguments
pub fn add_microdroid_payload_images(
    config: &VirtualMachineAppConfig,
    debug_config: &DebugConfig,
    temporary_directory: &Path,
    apk_file: File,
    idsig_file: File,
    extra_apk_files: Vec<File>,
    vm_payload_config: &VmPayloadConfig,
    vm_config: &mut VirtualMachineRawConfig,
) -> Result<()> {
    vm_config.disks.push(make_payload_disk(
        config,
        debug_config,
        apk_file,
        idsig_file,
        extra_apk_files,
        vm_payload_config,
        temporary_directory,
    )?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::NamedTempFile;

    #[test]
    fn test_find_apex_names_in_classpath() {
        let vars = r#"
export FOO /apex/unterminated
export BAR /apex/valid.apex/something
wrong
export EMPTY
export OTHER /foo/bar:/baz:/apex/second.valid.apex/:gibberish:"#;
        let expected = vec!["valid.apex", "second.valid.apex"];
        let expected: HashSet<_> = expected.into_iter().map(ToString::to_string).collect();

        assert_eq!(find_apex_names_in_classpath(vars).unwrap(), expected);
    }

    #[test]
    fn test_collect_apexes() -> Result<()> {
        let apex_infos_for_test = [
            (
                "adbd",
                ApexInfo {
                    name: "com.android.adbd".to_string(),
                    path: PathBuf::from("adbd"),
                    preinstalled_path: PathBuf::from("/system/adbd"),
                    has_classpath_jar: false,
                    last_update_seconds: 12345678,
                    is_factory: true,
                    is_active: false,
                    ..Default::default()
                },
            ),
            (
                "adbd_updated",
                ApexInfo {
                    name: "com.android.adbd".to_string(),
                    path: PathBuf::from("adbd"),
                    preinstalled_path: PathBuf::from("/system/adbd"),
                    has_classpath_jar: false,
                    last_update_seconds: 12345678 + 1,
                    is_factory: false,
                    is_active: true,
                    ..Default::default()
                },
            ),
            (
                "no_classpath",
                ApexInfo {
                    name: "no_classpath".to_string(),
                    path: PathBuf::from("no_classpath"),
                    has_classpath_jar: false,
                    last_update_seconds: 12345678,
                    is_factory: true,
                    is_active: true,
                    ..Default::default()
                },
            ),
            (
                "has_classpath",
                ApexInfo {
                    name: "has_classpath".to_string(),
                    path: PathBuf::from("has_classpath"),
                    has_classpath_jar: true,
                    last_update_seconds: 87654321,
                    is_factory: true,
                    is_active: false,
                    ..Default::default()
                },
            ),
            (
                "has_classpath_updated",
                ApexInfo {
                    name: "has_classpath".to_string(),
                    path: PathBuf::from("has_classpath/updated"),
                    preinstalled_path: PathBuf::from("/system/has_classpath"),
                    has_classpath_jar: true,
                    last_update_seconds: 87654321 + 1,
                    is_factory: false,
                    is_active: true,
                    ..Default::default()
                },
            ),
            (
                "apex-foo",
                ApexInfo {
                    name: "apex-foo".to_string(),
                    path: PathBuf::from("apex-foo"),
                    preinstalled_path: PathBuf::from("/system/apex-foo"),
                    has_classpath_jar: false,
                    last_update_seconds: 87654321,
                    is_factory: true,
                    is_active: false,
                    ..Default::default()
                },
            ),
            (
                "apex-foo-updated",
                ApexInfo {
                    name: "apex-foo".to_string(),
                    path: PathBuf::from("apex-foo/updated"),
                    preinstalled_path: PathBuf::from("/system/apex-foo"),
                    has_classpath_jar: false,
                    last_update_seconds: 87654321 + 1,
                    is_factory: false,
                    is_active: true,
                    ..Default::default()
                },
            ),
            (
                "sharedlibs",
                ApexInfo {
                    name: "sharedlibs".to_string(),
                    path: PathBuf::from("apex-foo"),
                    preinstalled_path: PathBuf::from("/system/apex-foo"),
                    last_update_seconds: 87654321,
                    is_factory: true,
                    provide_shared_apex_libs: true,
                    ..Default::default()
                },
            ),
            (
                "sharedlibs-updated",
                ApexInfo {
                    name: "sharedlibs".to_string(),
                    path: PathBuf::from("apex-foo/updated"),
                    preinstalled_path: PathBuf::from("/system/apex-foo"),
                    last_update_seconds: 87654321 + 1,
                    is_active: true,
                    provide_shared_apex_libs: true,
                    ..Default::default()
                },
            ),
        ];
        let apex_info_list = ApexInfoList {
            list: apex_infos_for_test.iter().map(|(_, info)| info).cloned().collect(),
        };
        let apex_info_map = HashMap::from(apex_infos_for_test);
        let apex_configs = vec![
            ApexConfig { name: "apex-foo".to_string() },
            ApexConfig { name: "{CLASSPATH}".to_string() },
        ];
        assert_eq!(
            collect_apex_infos(
                &apex_info_list,
                &apex_configs,
                &DebugConfig::new_with_debug_level(DebugLevel::FULL)
            )?,
            vec![
                // Pass active/required APEXes
                &apex_info_map["adbd_updated"],
                // Pass active APEXes specified in the config
                &apex_info_map["has_classpath_updated"],
                &apex_info_map["apex-foo-updated"],
                // Pass both preinstalled(inactive) and updated(active) for "sharedlibs" APEXes
                &apex_info_map["sharedlibs"],
                &apex_info_map["sharedlibs-updated"],
            ]
        );
        Ok(())
    }

    #[test]
    fn test_check_allowed_partitions_vendor_not_allowed() -> Result<()> {
        let apex_info_list = ApexInfoList {
            list: vec![ApexInfo {
                name: "apex-vendor".to_string(),
                path: PathBuf::from("apex-vendor"),
                preinstalled_path: PathBuf::from("/vendor/apex-vendor"),
                is_active: true,
                ..Default::default()
            }],
        };
        let apex_configs = vec![ApexConfig { name: "apex-vendor".to_string() }];

        let ret = collect_apex_infos(
            &apex_info_list,
            &apex_configs,
            &DebugConfig::new_with_debug_level(DebugLevel::NONE),
        );
        assert!(ret
            .is_err_and(|ret| ret.to_string()
                == "Non-system APEX apex-vendor is not supported in Microdroid"));

        Ok(())
    }

    #[test]
    fn test_check_allowed_partitions_system_ext_allowed() -> Result<()> {
        let apex_info_list = ApexInfoList {
            list: vec![ApexInfo {
                name: "apex-system_ext".to_string(),
                path: PathBuf::from("apex-system_ext"),
                preinstalled_path: PathBuf::from("/system_ext/apex-system_ext"),
                is_active: true,
                ..Default::default()
            }],
        };

        let apex_configs = vec![ApexConfig { name: "apex-system_ext".to_string() }];

        assert_eq!(
            collect_apex_infos(
                &apex_info_list,
                &apex_configs,
                &DebugConfig::new_with_debug_level(DebugLevel::NONE)
            )?,
            vec![&apex_info_list.list[0]]
        );

        Ok(())
    }

    #[test]
    fn test_prefer_staged_apex_with_factory_active_apex() {
        let single_apex = ApexInfo {
            name: "foo".to_string(),
            version: 1,
            path: PathBuf::from("foo.apex"),
            is_factory: true,
            is_active: true,
            ..Default::default()
        };
        let mut apex_info_list = ApexInfoList { list: vec![single_apex.clone()] };

        let staged = NamedTempFile::new().unwrap();
        apex_info_list
            .override_staged_apex(&StagedApexInfo {
                moduleName: "foo".to_string(),
                versionCode: 2,
                diskImagePath: staged.path().to_string_lossy().to_string(),
                ..Default::default()
            })
            .expect("should be ok");

        assert_eq!(
            apex_info_list,
            ApexInfoList {
                list: vec![
                    ApexInfo {
                        version: 2,
                        is_factory: false,
                        path: staged.path().to_owned(),
                        last_update_seconds: last_updated(staged.path()).unwrap(),
                        ..single_apex.clone()
                    },
                    ApexInfo { is_active: false, ..single_apex },
                ],
            }
        );
    }

    #[test]
    fn test_prefer_staged_apex_with_factory_and_inactive_apex() {
        let factory_apex = ApexInfo {
            name: "foo".to_string(),
            version: 1,
            path: PathBuf::from("foo.apex"),
            is_factory: true,
            ..Default::default()
        };
        let active_apex = ApexInfo {
            name: "foo".to_string(),
            version: 2,
            path: PathBuf::from("foo.downloaded.apex"),
            is_active: true,
            ..Default::default()
        };
        let mut apex_info_list =
            ApexInfoList { list: vec![factory_apex.clone(), active_apex.clone()] };

        let staged = NamedTempFile::new().unwrap();
        apex_info_list
            .override_staged_apex(&StagedApexInfo {
                moduleName: "foo".to_string(),
                versionCode: 3,
                diskImagePath: staged.path().to_string_lossy().to_string(),
                ..Default::default()
            })
            .expect("should be ok");

        assert_eq!(
            apex_info_list,
            ApexInfoList {
                list: vec![
                    // factory apex isn't touched
                    factory_apex,
                    // update active one
                    ApexInfo {
                        version: 3,
                        path: staged.path().to_owned(),
                        last_update_seconds: last_updated(staged.path()).unwrap(),
                        ..active_apex
                    },
                ],
            }
        );
    }
}
