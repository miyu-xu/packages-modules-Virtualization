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

//! Android VM control tool.

mod create_idsig;
mod create_partition;
mod run;

use android_system_virtualizationservice::aidl::android::system::virtualizationservice::{
    CpuTopology::CpuTopology, IVirtualizationService::IVirtualizationService,
    PartitionType::PartitionType, VirtualMachineAppConfig::DebugLevel::DebugLevel,
};
#[cfg(not(llpvm_changes))]
use anyhow::anyhow;
use anyhow::{bail, Context, Error};
use binder::{ProcessState, Strong};
use clap::{Args, Parser};
use create_idsig::command_create_idsig;
use create_partition::command_create_partition;
use run::{command_run, command_run_app, command_run_microdroid};
use serde::Serialize;
#[cfg(not(target_os = "android"))]
use std::fs::OpenOptions;
use std::io;
#[cfg(unix)]
use std::io::IsTerminal;
#[cfg(not(target_os = "android"))]
use std::io::{BufRead, Read, Seek, SeekFrom, Write};
use std::num::NonZeroU16;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
#[cfg(unix)]
use std::process::Command;
#[cfg(not(target_os = "android"))]
use std::thread;
#[cfg(not(target_os = "android"))]
use std::time::{Duration, Instant};

#[cfg(not(target_os = "android"))]
const WINDOWS_FILE_CONSOLE_PREFIX: &str = "win-file-console|";

#[cfg(all(unix, not(target_os = "android")))]
fn attach_unix_console(host_console_name: &str, forward_stdin: bool) -> Result<(), Error> {
    let console = OpenOptions::new()
        .read(true)
        .write(true)
        .open(host_console_name)
        .with_context(|| format!("Failed to open console tty {host_console_name}"))?;
    let mut console_reader = console
        .try_clone()
        .with_context(|| format!("Failed to clone console tty {host_console_name}"))?;
    let input_thread = forward_stdin.then(|| {
        thread::spawn(move || -> io::Result<()> {
            let mut stdin = io::stdin().lock();
            let mut console_writer = console;
            let _ = io::copy(&mut stdin, &mut console_writer)?;
            Ok(())
        })
    });

    let mut stdout = io::stdout().lock();
    io::copy(&mut console_reader, &mut stdout)
        .with_context(|| format!("Failed to read console tty {host_console_name}"))?;
    stdout.flush().context("Failed to flush console output")?;
    if let Some(input_thread) = input_thread {
        let _ = input_thread.join();
    }
    Ok(())
}

struct ConnectedService {
    _virtmgr: vmclient::VirtualizationService,
    service: Strong<dyn IVirtualizationService>,
}

impl ConnectedService {
    fn as_ref(&self) -> &dyn IVirtualizationService {
        self.service.as_ref()
    }
}

#[derive(Args, Default)]
/// Collection of flags that are at VM level and therefore applicable to all subcommands
pub struct CommonConfig {
    /// Name of VM
    #[arg(long)]
    name: Option<String>,

    /// Run VM with vCPU topology matching that of the host. If unspecified, defaults to 1 vCPU.
    #[arg(long, default_value = "one_cpu", value_parser = parse_cpu_topology)]
    cpu_topology: CpuTopology,

    /// Memory size (in MiB) of the VM. If unspecified, defaults to the value of `memory_mib`
    /// in the VM config file.
    #[arg(short, long)]
    mem: Option<u32>,

    /// Run VM in protected mode.
    #[arg(short, long)]
    protected: bool,

    /// Ask the kernel for transparent huge-pages (THP). This is only a hint and
    /// the kernel will allocate THP-backed memory only if globally enabled by
    /// the system and if any can be found. See
    /// https://docs.kernel.org/admin-guide/mm/transhuge.html
    #[arg(short, long)]
    hugepages: bool,

    /// Run VM with network feature.
    #[cfg(network)]
    #[arg(short, long)]
    network_supported: bool,

    /// Boost uclamp to stablise results for benchmarks.
    #[arg(short, long)]
    boost_uclamp: bool,
}

impl CommonConfig {
    fn network_supported(&self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(network)] {
                self.network_supported
            } else {
                false
            }
        }
    }
}

#[derive(Args, Default)]
/// Collection of flags for debugging
pub struct DebugConfig {
    /// Debug level of the VM. Supported values: "full" (default), and "none".
    #[arg(long, default_value = "full", value_parser = parse_debug_level)]
    debug: DebugLevel,

    /// Path to file for VM console output.
    #[arg(long)]
    console: Option<PathBuf>,

    /// Path to file for VM console input.
    #[arg(long)]
    console_in: Option<PathBuf>,

    /// Path to file for VM log output.
    #[arg(long)]
    log: Option<PathBuf>,

    /// Port at which crosvm will start a gdb server to debug guest kernel.
    /// Note: this is only supported on Android kernels android14-5.15 and higher.
    #[arg(long)]
    gdb: Option<NonZeroU16>,

    /// Listen on localhost:<port> and bridge accepted TCP clients to guest vsock:5555.
    #[cfg(not(target_os = "android"))]
    #[arg(long)]
    adb_tcp_port: Option<NonZeroU16>,

    /// Whether to enable earlycon. Only supported for debuggable Linux-based VMs.
    #[arg(long)]
    enable_earlycon: bool,
}

impl DebugConfig {
    fn enable_earlycon(&self) -> bool {
        self.enable_earlycon
    }

    fn adb_tcp_port(&self) -> Option<u16> {
        cfg_if::cfg_if! {
            if #[cfg(not(target_os = "android"))] {
                self.adb_tcp_port.map(NonZeroU16::get)
            } else {
                None
            }
        }
    }
}

#[derive(Args, Default)]
/// Collection of flags that are Microdroid specific
pub struct MicrodroidConfig {
    /// Path to the file backing the storage.
    /// Created if the option is used but the path does not exist in the device.
    #[arg(long)]
    storage: Option<PathBuf>,

    /// Size of the storage. Used only if --storage is supplied but path does not exist
    /// Default size is 10*1024*1024
    #[arg(long)]
    storage_size: Option<u64>,

    /// Path to disk image containing vendor-specific modules.
    #[cfg(vendor_modules)]
    #[arg(long)]
    vendor: Option<PathBuf>,

    /// SysFS nodes of devices to assign to VM
    #[cfg(device_assignment)]
    #[arg(long)]
    devices: Vec<PathBuf>,

    /// Version of GKI to use. If set, use instead of microdroid kernel
    #[cfg(vendor_modules)]
    #[arg(long)]
    gki: Option<String>,
}

impl MicrodroidConfig {
    fn vendor(&self) -> Option<&PathBuf> {
        cfg_if::cfg_if! {
            if #[cfg(vendor_modules)] {
                self.vendor.as_ref()
            } else {
                None
            }
        }
    }

    fn gki(&self) -> Option<&str> {
        cfg_if::cfg_if! {
            if #[cfg(vendor_modules)] {
                self.gki.as_deref()
            } else {
                None
            }
        }
    }

    fn devices(&self) -> &[PathBuf] {
        cfg_if::cfg_if! {
            if #[cfg(device_assignment)] {
                &self.devices
            } else {
                &[]
            }
        }
    }
}

#[derive(Args, Default)]
/// Flags for the run_app subcommand
pub struct RunAppConfig {
    #[command(flatten)]
    common: CommonConfig,

    #[command(flatten)]
    debug: DebugConfig,

    #[command(flatten)]
    microdroid: MicrodroidConfig,

    /// Path to VM Payload APK
    apk: PathBuf,

    /// Path to idsig of the APK
    idsig: PathBuf,

    /// Path to the instance image. Created if not exists.
    instance: PathBuf,

    /// Path to file containing instance_id. Required iff llpvm feature is enabled.
    #[cfg(llpvm_changes)]
    #[arg(long = "instance-id-file")]
    instance_id: PathBuf,

    /// Path to VM config JSON within APK (e.g. assets/vm_config.json)
    #[arg(long)]
    config_path: Option<String>,

    /// Name of VM payload binary within APK (e.g. MicrodroidTestNativeLib.so)
    #[arg(long)]
    #[arg(alias = "payload_path")]
    payload_binary_name: Option<String>,

    /// Paths to extra apk files.
    #[cfg(multi_tenant)]
    #[arg(long = "extra-apk")]
    #[clap(conflicts_with = "config_path")]
    extra_apks: Vec<PathBuf>,

    /// Paths to extra idsig files.
    #[arg(long = "extra-idsig")]
    extra_idsigs: Vec<PathBuf>,
}

impl RunAppConfig {
    fn extra_apks(&self) -> &[PathBuf] {
        cfg_if::cfg_if! {
            if #[cfg(multi_tenant)] {
                &self.extra_apks
            } else {
                &[]
            }
        }
    }

    fn instance_id(&self) -> Result<PathBuf, Error> {
        cfg_if::cfg_if! {
            if #[cfg(llpvm_changes)] {
                Ok(self.instance_id.clone())
            } else {
                Err(anyhow!("LLPVM feature is disabled, --instance_id flag not supported"))
            }
        }
    }

    fn set_instance_id(&mut self, instance_id_file: PathBuf) -> Result<(), Error> {
        cfg_if::cfg_if! {
            if #[cfg(llpvm_changes)] {
                self.instance_id = instance_id_file;
                Ok(())
            } else {
                let _ = instance_id_file;
                Err(anyhow!("LLPVM feature is disabled, --instance_id flag not supported"))
            }
        }
    }
}

#[derive(Args, Default)]
/// Flags for the run_microdroid subcommand
pub struct RunMicrodroidConfig {
    #[command(flatten)]
    common: CommonConfig,

    #[command(flatten)]
    debug: DebugConfig,

    #[command(flatten)]
    microdroid: MicrodroidConfig,

    /// Path to the directory where VM-related files (e.g. instance.img, apk.idsig, etc.) will
    /// be stored. If not specified a random directory under /data/local/tmp/microdroid will be
    /// created and used.
    #[arg(long)]
    work_dir: Option<PathBuf>,
}

#[derive(Args, Default)]
/// Flags for the run subcommand
pub struct RunCustomVmConfig {
    #[command(flatten)]
    common: CommonConfig,

    #[command(flatten)]
    debug: DebugConfig,

    /// Path to VM config JSON
    config: PathBuf,
}

#[derive(Parser)]
enum Opt {
    /// Check if the feature is enabled on device.
    CheckFeatureEnabled { feature: String },
    /// Run a virtual machine with a config in APK
    RunApp {
        #[command(flatten)]
        config: RunAppConfig,
    },
    /// Run a virtual machine with Microdroid inside
    RunMicrodroid {
        #[command(flatten)]
        config: RunMicrodroidConfig,
    },
    /// Run a virtual machine
    Run {
        #[command(flatten)]
        config: RunCustomVmConfig,
    },
    /// List running virtual machines
    List,
    /// Print information about virtual machine support
    Info,
    /// Create a new empty partition to be used as a writable partition for a VM
    CreatePartition {
        /// Path at which to create the image file
        path: PathBuf,

        /// The desired size of the partition, in bytes.
        size: u64,

        /// Type of the partition
        #[arg(short = 't', long = "type", default_value = "raw",
               value_parser = parse_partition_type)]
        partition_type: PartitionType,
    },
    /// Creates or update the idsig file by digesting the input APK file.
    CreateIdsig {
        /// Path to VM Payload APK
        apk: PathBuf,

        /// Path to idsig of the APK
        path: PathBuf,
    },
    /// Connect to the serial console of a VM
    Console {
        /// CID of the VM
        cid: Option<i32>,

        /// Exit after reading console output for the given number of seconds.
        #[cfg(not(target_os = "android"))]
        #[arg(long)]
        timeout_secs: Option<u64>,

        /// Do not forward local stdin into the VM console input file.
        #[cfg(not(target_os = "android"))]
        #[arg(long)]
        read_only: bool,
    },
}

fn parse_debug_level(s: &str) -> Result<DebugLevel, String> {
    match s {
        "none" => Ok(DebugLevel::NONE),
        "full" => Ok(DebugLevel::FULL),
        _ => Err(format!("Invalid debug level {}", s)),
    }
}

fn parse_partition_type(s: &str) -> Result<PartitionType, String> {
    match s {
        "raw" => Ok(PartitionType::RAW),
        "instance" => Ok(PartitionType::ANDROID_VM_INSTANCE),
        _ => Err(format!("Invalid partition type {}", s)),
    }
}

fn parse_cpu_topology(s: &str) -> Result<CpuTopology, String> {
    match s {
        "one_cpu" => Ok(CpuTopology::ONE_CPU),
        "match_host" => Ok(CpuTopology::MATCH_HOST),
        _ => Err(format!("Invalid cpu topology {}", s)),
    }
}

fn get_service() -> Result<ConnectedService, Error> {
    let virtmgr =
        vmclient::VirtualizationService::new().context("Failed to spawn VirtualizationService")?;
    let service = virtmgr.connect().context("Failed to connect to VirtualizationService")?;
    Ok(ConnectedService { _virtmgr: virtmgr, service })
}

fn command_check_feature_enabled(feature: &str) {
    println!(
        "Feature {feature} is {}",
        if avf_features::is_feature_enabled(feature) { "enabled" } else { "disabled" }
    );
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let opt = Opt::parse();

    #[cfg(not(windows))]
    {
        // We need to start the thread pool for Binder to work properly, especially link_to_death.
        ProcessState::start_thread_pool();
    }

    match opt {
        Opt::CheckFeatureEnabled { feature } => {
            command_check_feature_enabled(&feature);
            Ok(())
        }
        Opt::RunApp { config } => command_run_app(config),
        Opt::RunMicrodroid { config } => command_run_microdroid(config),
        Opt::Run { config } => command_run(config),
        Opt::List => {
            let service = get_service()?;
            command_list(service.as_ref())
        }
        Opt::Info => command_info(),
        Opt::CreatePartition { path, size, partition_type } => {
            let service = get_service()?;
            command_create_partition(service.as_ref(), &path, size, partition_type)
        }
        Opt::CreateIdsig { apk, path } => {
            let service = get_service()?;
            command_create_idsig(service.as_ref(), &apk, &path)
        }
        Opt::Console {
            cid,
            #[cfg(not(target_os = "android"))]
            timeout_secs,
            #[cfg(not(target_os = "android"))]
            read_only,
        } => {
            #[cfg(not(target_os = "android"))]
            {
                command_console(cid, timeout_secs, read_only)
            }
            #[cfg(target_os = "android")]
            {
                command_console(cid)
            }
        }
    }
}

/// List the VMs currently running.
fn command_list(service: &dyn IVirtualizationService) -> Result<(), Error> {
    let vms = service.debugListVms().context("Failed to get list of VMs")?;
    println!("Running VMs: {:#?}", vms);
    Ok(())
}

/// Print information about supported VM types.
fn command_info() -> Result<(), Error> {
    let non_protected_vm_supported = hypervisor_props::is_vm_supported()?;
    let protected_vm_supported = hypervisor_props::is_protected_vm_supported()?;
    match (non_protected_vm_supported, protected_vm_supported) {
        (false, false) => println!("VMs are not supported."),
        (false, true) => println!("Only protected VMs are supported."),
        (true, false) => println!("Only non-protected VMs are supported."),
        (true, true) => println!("Both protected and non-protected VMs are supported."),
    }

    if let Some(version) = hypervisor_props::version()? {
        println!("Hypervisor version: {}", version);
    } else {
        println!("Hypervisor version not set.");
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        use std::path::Path;
        if Path::new("/dev/kvm").exists() {
            println!("/dev/kvm exists.");
        } else {
            println!("/dev/kvm does not exist.");
        }

        if Path::new("/dev/vfio/vfio").exists() {
            println!("/dev/vfio/vfio exists.");
        } else {
            println!("/dev/vfio/vfio does not exist.");
        }

        if Path::new("/sys/bus/platform/drivers/vfio-platform").exists() {
            println!("VFIO-platform is supported.");
        } else {
            println!("VFIO-platform is not supported.");
        }
    }
    #[cfg(target_os = "macos")]
    {
        println!(
            "Hypervisor.framework (HVF): expected host hypervisor backend on macOS Apple Silicon."
        );
        println!("Linux device nodes (/dev/kvm, /dev/vfio/vfio, VFIO-platform sysfs): not applicable on macOS.");
    }
    #[cfg(windows)]
    {
        println!("Linux device nodes (/dev/kvm, /dev/vfio/vfio, VFIO-platform sysfs): not applicable on Windows.");
    }

    #[derive(Serialize)]
    struct AssignableDevice {
        node: String,
        dtbo_label: String,
    }

    let service = get_service()?;
    let devices = service.as_ref().getAssignableDevices()?;
    let devices: Vec<_> = devices
        .into_iter()
        .map(|device| AssignableDevice { node: device.node, dtbo_label: device.dtbo_label })
        .collect();
    println!("Assignable devices: {}", serde_json::to_string(&devices)?);

    let os_list = service.as_ref().getSupportedOSList()?;
    println!("Available OS list: {}", serde_json::to_string(&os_list)?);

    Ok(())
}

#[cfg(target_os = "android")]
fn command_console(cid: Option<i32>) -> Result<(), Error> {
    if !io::stdin().is_terminal() {
        bail!("Stdin must be a terminal (tty). Use 'adb shell -t' to force allocate tty.");
    }
    let service = get_service()?;
    let mut vms = service.as_ref().debugListVms().context("Failed to get list of VMs")?;
    if let Some(cid) = cid {
        vms.retain(|vm_info| vm_info.cid == cid);
    }
    let host_console_name = vms
        .into_iter()
        .find_map(|vm_info| vm_info.hostConsoleName)
        .context("Failed to get VM with console")?;
    Err(Command::new("microcom").arg(host_console_name).exec().into())
}

#[cfg(windows)]
#[derive(Debug)]
struct WindowsConsoleInfo {
    output_path: PathBuf,
    input_path: Option<PathBuf>,
}

#[cfg(windows)]
fn parse_windows_console_info(raw: Option<String>, temp_dir: &str) -> Option<WindowsConsoleInfo> {
    if let Some(raw) = raw {
        if let Some(rest) = raw.strip_prefix(WINDOWS_FILE_CONSOLE_PREFIX) {
            let mut parts = rest.splitn(2, '|');
            let output = parts.next()?.trim();
            if output.is_empty() {
                return None;
            }
            let input = parts.next().map(str::trim).filter(|p| !p.is_empty()).map(PathBuf::from);
            return Some(WindowsConsoleInfo {
                output_path: PathBuf::from(output),
                input_path: input,
            });
        }

        if !raw.trim().is_empty() {
            return Some(WindowsConsoleInfo { output_path: PathBuf::from(raw), input_path: None });
        }
    }

    let output_path = PathBuf::from(temp_dir).join("guest-virtio-console1.txt");
    if output_path.exists() {
        return Some(WindowsConsoleInfo { output_path, input_path: None });
    }

    None
}

#[cfg(windows)]
fn forward_console_input(input_path: PathBuf) {
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut locked = stdin.lock();
        loop {
            let mut line = String::new();
            match locked.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if let Ok(mut file) =
                        OpenOptions::new().create(true).append(true).open(&input_path)
                    {
                        let _ = file.write_all(line.as_bytes());
                    }
                }
                Err(_) => break,
            }
        }
    });
}

#[cfg(windows)]
fn tail_console_output(output_path: &PathBuf, timeout_secs: Option<u64>) -> Result<(), Error> {
    let deadline = timeout_secs.map(|secs| Instant::now() + Duration::from_secs(secs));
    let mut offset = 0u64;

    loop {
        if let Some(deadline) = deadline {
            if Instant::now() >= deadline {
                break;
            }
        }

        if let Ok(mut file) = OpenOptions::new().read(true).open(output_path) {
            file.seek(SeekFrom::Start(offset))
                .with_context(|| format!("Failed to seek console file {:?}", output_path))?;
            let mut buf = Vec::new();
            let bytes_read = std::io::Read::read_to_end(&mut file, &mut buf)
                .with_context(|| format!("Failed to read console file {:?}", output_path))?;
            if bytes_read > 0 {
                offset += bytes_read as u64;
                print!("{}", String::from_utf8_lossy(&buf));
                io::stdout().flush().context("Failed to flush console output")?;
            }
        }

        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}

#[cfg(windows)]
fn command_console(
    cid: Option<i32>,
    timeout_secs: Option<u64>,
    read_only: bool,
) -> Result<(), Error> {
    let service = get_service()?;
    let mut vms = service.as_ref().debugListVms().context("Failed to get list of VMs")?;
    if let Some(cid) = cid {
        vms.retain(|vm_info| vm_info.cid == cid);
    }
    let vm_info = match vms.len() {
        0 => bail!("Failed to get VM with console"),
        1 => vms.remove(0),
        _ => {
            let cids: Vec<_> = vms.iter().map(|vm| vm.cid).collect();
            bail!("Multiple VMs are running; specify a CID with `vm console <cid>`. Available CIDs: {:?}", cids);
        }
    };

    if let Some(console) =
        parse_windows_console_info(vm_info.hostConsoleName.clone(), &vm_info.temporaryDirectory)
    {
        eprintln!("Connecting to host VM console for CID {}", vm_info.cid);
        eprintln!("Console output: {}", console.output_path.display());
        if let Some(input_path) = &console.input_path {
            eprintln!("Console input : {}", input_path.display());
        }

        if !read_only {
            if let Some(input_path) = console.input_path.clone() {
                forward_console_input(input_path);
            }
        }

        return tail_console_output(&console.output_path, timeout_secs);
    }

    #[cfg(unix)]
    {
        if !io::stdin().is_terminal() {
            bail!("Stdin must be a terminal (tty).");
        }
        let host_console_name =
            vm_info.hostConsoleName.context("Failed to get host console metadata for the VM")?;
        Err(Command::new("microcom").arg(host_console_name).exec().into())
    }
    #[cfg(windows)]
    {
        bail!("Failed to resolve host console metadata for the VM")
    }
}

#[cfg(all(unix, not(target_os = "android")))]
fn command_console(
    cid: Option<i32>,
    _timeout_secs: Option<u64>,
    _read_only: bool,
) -> Result<(), Error> {
    let forward_stdin = io::stdin().is_terminal();
    let service = get_service()?;
    let mut vms = service.as_ref().debugListVms().context("Failed to get list of VMs")?;
    if let Some(cid) = cid {
        vms.retain(|vm_info| vm_info.cid == cid);
    }
    let vm_info = match vms.len() {
        0 => bail!("Failed to get VM with console"),
        1 => vms.remove(0),
        _ => {
            let cids: Vec<_> = vms.iter().map(|vm| vm.cid).collect();
            bail!(
                "Multiple VMs are running; specify a CID with `vm console <cid>`. Available CIDs: {:?}",
                cids
            );
        }
    };
    let host_console_name =
        vm_info.hostConsoleName.context("Failed to get host console metadata for the VM")?;
    eprintln!("Connecting to host VM console for CID {}", vm_info.cid);
    eprintln!("Console tty   : {}", host_console_name);
    if !forward_stdin {
        eprintln!("Local stdin is not a tty; attaching in output-only mode.");
    }
    attach_unix_console(&host_console_name, forward_stdin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_app() {
        // Check that the command parsing has been configured in a valid way.
        Opt::command().debug_assert();
    }
}
