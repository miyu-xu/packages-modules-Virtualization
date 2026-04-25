use android_system_virtualizationservice::aidl::android::system::virtualizationservice::{
    CpuTopology::CpuTopology, DiskImage::DiskImage, Partition::Partition,
    VirtualMachineAppConfig::DebugLevel::DebugLevel, VirtualMachineConfig::VirtualMachineConfig,
    VirtualMachineRawConfig::VirtualMachineRawConfig,
};
use anyhow::{anyhow, Context, Result};
use binder::ParcelFileDescriptor;
use serde::Deserialize;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct VmConfig {
    #[serde(default)]
    name: String,
    #[serde(default)]
    cpu_topology: Option<String>,
    #[serde(default)]
    platform_version: Option<String>,
    #[serde(default)]
    memory_mib: Option<i32>,
    #[serde(default)]
    console_input_device: Option<String>,
    #[serde(default)]
    bootloader: Option<String>,
    #[serde(default)]
    kernel: Option<String>,
    #[serde(default)]
    initrd: Option<String>,
    #[serde(default)]
    params: Option<String>,
    #[serde(default)]
    disks: Vec<DiskJson>,
    #[serde(default, rename = "protected")]
    protected_vm: bool,
}

#[derive(Debug, Deserialize)]
struct DiskJson {
    #[serde(default)]
    writable: bool,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    partitions: Vec<PartitionJson>,
}

#[derive(Debug, Deserialize)]
struct PartitionJson {
    #[serde(default)]
    writable: bool,
    label: String,
    path: String,
    #[serde(default)]
    guid: Option<String>,
}

impl VmConfig {
    pub fn load(f: &File) -> Result<Self> {
        let cfg: VmConfig =
            serde_json::from_reader(f).context("Failed to parse VM config JSON into VmConfig")?;
        Ok(cfg)
    }

    pub fn to_parcelable(&self) -> Result<VirtualMachineRawConfig> {
        let mut out = VirtualMachineRawConfig {
            name: self.name.clone(),
            protectedVm: self.protected_vm,
            params: self.params.clone(),
            platformVersion: self.platform_version.clone().unwrap_or_default(),
            consoleInputDevice: self.console_input_device.clone(),
            ..Default::default()
        };
        if let Some(m) = self.memory_mib {
            out.memoryMib = m;
        }
        out.cpuTopology = match self.cpu_topology.as_deref() {
            Some("match_host") => CpuTopology::MATCH_HOST,
            Some("one_cpu") | None => CpuTopology::ONE_CPU,
            Some(other) => return Err(anyhow!("invalid cpu_topology: {other}")),
        };
        out.kernel =
            self.kernel.as_deref().map(|p| open_parcel_file(Path::new(p), false)).transpose()?;
        out.initrd =
            self.initrd.as_deref().map(|p| open_parcel_file(Path::new(p), false)).transpose()?;
        out.bootloader = self
            .bootloader
            .as_deref()
            .map(|p| open_parcel_file(Path::new(p), false))
            .transpose()?;
        out.disks = self
            .disks
            .iter()
            .map(|disk| {
                let image = disk
                    .image
                    .as_deref()
                    .map(|p| open_parcel_file(Path::new(p), disk.writable))
                    .transpose()?;
                let partitions = disk
                    .partitions
                    .iter()
                    .map(|part| {
                        Ok(Partition {
                            label: part.label.clone(),
                            image: Some(open_parcel_file(
                                Path::new(&part.path),
                                disk.writable && part.writable,
                            )?),
                            writable: disk.writable && part.writable,
                            guid: part.guid.clone(),
                        })
                    })
                    .collect::<Result<Vec<Partition>>>()?;
                Ok(DiskImage { image, writable: disk.writable, partitions })
            })
            .collect::<Result<Vec<DiskImage>>>()?;
        Ok(out)
    }
}

pub fn get_debug_level(config: &VirtualMachineConfig) -> Option<DebugLevel> {
    match config {
        VirtualMachineConfig::AppConfig(c) => Some(c.debugLevel),
        VirtualMachineConfig::RawConfig(_c) => None,
    }
}

pub fn open_parcel_file(path: &Path, _writable: bool) -> Result<ParcelFileDescriptor> {
    let resolved = resolve_host_path(path);
    let f = OpenOptions::new().read(true).write(_writable).open(&resolved).with_context(|| {
        format!("Failed to open path {} (resolved: {})", path.display(), resolved.display())
    })?;
    Ok(ParcelFileDescriptor::new(f))
}

pub fn resolve_host_path(path: &Path) -> PathBuf {
    #[cfg(not(target_os = "android"))]
    {
        if path.exists() {
            return path.to_path_buf();
        }
        let raw = path.to_string_lossy();
        #[cfg(windows)]
        let android_root = std::env::var("VIRTMGR_ANDROID_ROOT")
            .unwrap_or_else(|_| "C:/workspace/aosp".to_string());
        let map_prefix = |prefix: &str, env_key: &str| -> Option<PathBuf> {
            #[cfg(windows)]
            let root = std::env::var(env_key).unwrap_or_else(|_| format!("{android_root}{prefix}"));
            #[cfg(not(windows))]
            let root = std::env::var(env_key).ok()?;
            raw.strip_prefix(prefix).map(|rest| {
                let rest = rest.trim_start_matches('/');
                Path::new(&root).join(rest)
            })
        };
        if let Some(mapped) = map_prefix("/apex", "VIRTMGR_APEX_ROOT") {
            return mapped;
        }
        if let Some(mapped) = map_prefix("/system_ext", "VIRTMGR_SYSTEM_EXT_ROOT") {
            return mapped;
        }
        if let Some(mapped) = map_prefix("/system", "VIRTMGR_SYSTEM_ROOT") {
            return mapped;
        }
        path.to_path_buf()
    }
    #[cfg(target_os = "android")]
    {
        path.to_path_buf()
    }
}
