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

//! Launch and manage `crosvm` on Windows using path-based arguments and a named pipe for VM
//! control (`--socket`). Requires a working `crosvm.exe` (see `VIRTMGR_CROSVM_PATH`).

use crate::aidl::{debug_trace, remove_temporary_files, Cid, VirtualMachineCallbacks};
use crate::atom::{get_num_cpus, write_vm_exited_stats_sync};
use crate::debug_config::DebugConfig;
use anyhow::{anyhow, bail, Context, Error, Result};
use android_system_virtualizationcommon::aidl::android::system::virtualizationcommon::DeathReason::DeathReason;
use android_system_virtualizationservice::aidl::android::system::virtualizationservice::{
    AudioConfig::AudioConfig as AudioConfigParcelable,
    DisplayConfig::DisplayConfig as DisplayConfigParcelable,
    GpuConfig::GpuConfig as GpuConfigParcelable,
    UsbConfig::UsbConfig as UsbConfigParcelable,
    VirtualMachineAppConfig::DebugLevel::DebugLevel,
};
use android_system_virtualizationservice_internal::aidl::android::system::virtualizationservice_internal::IGlobalVmContext::IGlobalVmContext;
use android_system_virtualizationservice_internal::aidl::android::system::virtualizationservice_internal::IBoundDevice::IBoundDevice;
use android_system_virtualmachineservice::aidl::android::system::virtualmachineservice::IVirtualMachineService::IVirtualMachineService;
use binder::Strong;
use log::{debug, error, info, warn};
use rpcbinder::RpcServer;
use semver::Version;
use semver::VersionReq;
use shared_child::SharedChild;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::mem;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::num::{NonZeroU16, NonZeroU32};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use vm_control::{BalloonControlCommand, VmRequest, VmResponse};
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle, RawHandle};
use std::ptr;
use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
use windows_sys::Win32::Storage::FileSystem::{
    GetFinalPathNameByHandleW, FILE_FLAG_OVERLAPPED, PIPE_ACCESS_INBOUND,
    PIPE_ACCESS_OUTBOUND,
};
use windows_sys::Win32::System::Console::{GetConsoleMode, GetNumberOfConsoleInputEvents};
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, PIPE_READMODE_BYTE, PIPE_TYPE_BYTE, PIPE_WAIT,
};



type VfioDevice = Strong<dyn IBoundDevice>;

/// Environment variable overriding the `crosvm` executable used to run and control VMs.
pub const VIRTMGR_CROSVM_PATH: &str = "VIRTMGR_CROSVM_PATH";
const VIRTMGR_CAPTURE_GUEST_CONSOLE: &str = "VIRTMGR_CAPTURE_GUEST_CONSOLE";
const VIRTMGR_CAPTURE_CROSVM_STDIO: &str = "VIRTMGR_CAPTURE_CROSVM_STDIO";

const CROSVM_PLATFORM_VERSION: &str = "1.0.0";
const CROSVM_START_ERROR_STATUS: i32 = 1;
const CROSVM_REBOOT_STATUS: i32 = 32;
const CROSVM_CRASH_STATUS: i32 = 33;
const CROSVM_WATCHDOG_REBOOT_STATUS: i32 = 36;
const RAMDUMP_RESERVED_MIB: u32 = 17;
const MILLIS_PER_SEC: i64 = 1000;
const SYSPROP_CUSTOM_PVMFW_PATH: &str = "hypervisor.pvmfw.path";
const CONSOLE_HVC0: &str = "hvc0";
const CONSOLE_TTYS0: &str = "ttyS0";

static BOOT_HANGUP_TIMEOUT: once_cell::sync::Lazy<Duration> =
    // Windows bringup is slower than the Unix AVF path because guest->host Binder RPC,
    // composite disk assembly, and the userspace virtio-vsock bridge all sit on compatibility
    // layers instead of native Android plumbing.
    once_cell::sync::Lazy::new(|| Duration::from_secs(120));

fn crosvm_binary() -> OsString {
    std::env::var_os(VIRTMGR_CROSVM_PATH).unwrap_or_else(|| OsString::from("crosvm.exe"))
}

/// Best-effort path for a file handle (for crosvm `path=` arguments).
fn file_path_for_crosvm(file: &File) -> Result<PathBuf> {
    let h = file.as_raw_handle();
    let mut buf = vec![0u16; 32768];
    let len = unsafe { GetFinalPathNameByHandleW(h as _, buf.as_mut_ptr(), buf.len() as u32, 0) };
    if len == 0 {
        let mut console_mode = 0;
        if unsafe { GetConsoleMode(h as _, &mut console_mode) } != 0 {
            let mut pending_events = 0;
            let console_path =
                if unsafe { GetNumberOfConsoleInputEvents(h as _, &mut pending_events) } != 0 {
                    "CONIN$"
                } else {
                    "CONOUT$"
                };
            return Ok(PathBuf::from(console_path));
        }
        bail!("GetFinalPathNameByHandleW failed: {}", std::io::Error::last_os_error());
    }
    let os_str = OsString::from_wide(&buf[..len as usize]);
    let p = PathBuf::from(os_str);
    // Strip extended-length prefix for readability; crosvm accepts both.
    Ok(p)
}

fn add_path(file: File) -> Result<String> {
    let p = file_path_for_crosvm(&file)?;
    Ok(p.to_string_lossy().into_owned())
}

fn guest_console_capture_enabled() -> bool {
    std::env::var_os(VIRTMGR_CAPTURE_GUEST_CONSOLE)
        .map(|v| !v.is_empty() && v != "0")
        .unwrap_or(false)
}

fn crosvm_stdio_capture_enabled() -> bool {
    std::env::var_os(VIRTMGR_CAPTURE_CROSVM_STDIO)
        .map(|v| !v.is_empty() && v != "0")
        .unwrap_or(false)
}

/// Create a named pipe server for guest console output.
/// Returns the pipe path (for crosvm's `--serial`) and the server read handle.
/// Crosvm opens the client end via `CreateFile` when it processes `type=file,path=\\.\pipe\...`.
#[cfg(windows)]
fn create_console_named_pipe(cid: Cid) -> Result<(String, OwnedHandle)> {
    let pipe_name = format!(r"\\.\pipe\virtmgr_console_{cid}");
    let wide: Vec<u16> = pipe_name.encode_utf16().chain(std::iter::once(0)).collect();

    let handle = unsafe {
        CreateNamedPipeW(
            wide.as_ptr(),
            PIPE_ACCESS_INBOUND | FILE_FLAG_OVERLAPPED,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
            1,     // max one instance
            65536, // out buffer (crosvm writes into the pipe)
            65536, // in buffer (we read from the pipe)
            0,     // default timeout
            ptr::null_mut(),
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        bail!(
            "CreateNamedPipeW failed for console pipe: {}",
            std::io::Error::last_os_error()
        );
    }

    Ok((pipe_name, unsafe { OwnedHandle::from_raw_handle(handle as RawHandle) }))
}

/// Create a named pipe server for guest console input.
/// Returns the pipe path (for crosvm's `,input=` serial arg) and the server write handle.
/// Crosvm opens the client end and reads from this pipe; we write keyboard input to the server end.
#[cfg(windows)]
fn create_console_input_pipe(cid: Cid) -> Result<(String, OwnedHandle)> {
    let pipe_name = format!(r"\\.\pipe\virtmgr_console_input_{cid}");
    let wide: Vec<u16> = pipe_name.encode_utf16().chain(std::iter::once(0)).collect();

    let handle = unsafe {
        CreateNamedPipeW(
            wide.as_ptr(),
            PIPE_ACCESS_OUTBOUND | FILE_FLAG_OVERLAPPED,
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
            1,     // max one instance
            65536, // out buffer (we write into the pipe)
            65536, // in buffer (crosvm reads from the pipe)
            0,     // default timeout
            ptr::null_mut(),
        )
    };

    if handle == INVALID_HANDLE_VALUE {
        bail!(
            "CreateNamedPipeW failed for console input pipe: {}",
            std::io::Error::last_os_error()
        );
    }

    Ok((pipe_name, unsafe { OwnedHandle::from_raw_handle(handle as RawHandle) }))
}

/// Read loop for a named pipe console: accepts the connection, reads data,
/// and appends to a shared buffer. Runs until the pipe is disconnected.
#[cfg(windows)]
fn console_pipe_read_loop(
    pipe_handle: OwnedHandle,
    buffer: Arc<Mutex<Vec<u8>>>,
    mut output: Option<File>,
) {
    let raw = pipe_handle.as_raw_handle() as _;

    // Accept the client connection (crosvm opens our pipe via CreateFile)
    let connected = unsafe { ConnectNamedPipe(raw, ptr::null_mut()) };
    if connected == 0 {
        let err = std::io::Error::last_os_error();
        // ERROR_PIPE_CONNECTED (535) means the client already connected
        if err.raw_os_error() != Some(535) {
            warn!("ConnectNamedPipe for console pipe failed: {err}");
            return;
        }
    }
    info!("Console named pipe connected");

    let mut buf = [0u8; 65536];
    loop {
        let mut total = 0u32;
        let rc = unsafe {
            windows_sys::Win32::Storage::FileSystem::ReadFile(
                raw,
                buf.as_mut_ptr() as _,
                buf.len() as u32,
                &mut total,
                ptr::null_mut(),
            )
        };
        if rc == 0 {
            // Pipe disconnected or error — crosvm closed its end
            break;
        }
        if total > 0 {
            let bytes = &buf[..total as usize];
            let write_result = output
                .as_mut()
                .map(|file| file.write_all(bytes).and_then(|()| file.flush()));
            if let Some(Err(error)) = write_result {
                warn!("Failed to persist guest console output: {error}");
                output = None;
            }
            buffer.lock().unwrap().extend_from_slice(bytes);
        }
    }
    info!("Console named pipe closed");
}

fn format_serial_out_arg(path: Option<File>) -> Result<String> {
    if let Some(file) = path {
        Ok(format!("type=file,path={}", add_path(file)?))
    } else {
        Ok("type=sink".to_string())
    }
}

fn windows_input_arg(opt: InputDeviceOption) -> Result<String> {
    Ok(match opt {
        InputDeviceOption::EvDev(file) => format!("evdev[path={}]", add_path(file)?),
        InputDeviceOption::Keyboard(file) => format!("keyboard[path={}]", add_path(file)?),
        InputDeviceOption::Mouse(file) => format!("mouse[path={}]", add_path(file)?),
        InputDeviceOption::SingleTouch { file, width, height, name } => format!(
            "single-touch[path={},width={},height={}{}]",
            add_path(file)?,
            width,
            height,
            name.as_ref().map_or("".into(), |n| format!(",name={}", n))
        ),
        InputDeviceOption::Switches(file) => format!("switches[path={}]", add_path(file)?),
        InputDeviceOption::MultiTouchTrackpad { file, width, height, name } => format!(
            "multi-touch-trackpad[path={},width={},height={}{}]",
            add_path(file)?,
            width,
            height,
            name.as_ref().map_or("".into(), |n| format!(",name={}", n))
        ),
        InputDeviceOption::MultiTouch { file, width, height, name } => format!(
            "multi-touch[path={},width={},height={}{}]",
            add_path(file)?,
            width,
            height,
            name.as_ref().map_or("".into(), |n| format!(",name={}", n))
        ),
    })
}

/// Configuration for a VM to run with crosvm.
#[derive(Debug)]
pub struct CrosvmConfig {
    pub cid: Cid,
    pub name: String,
    pub bootloader: Option<File>,
    pub kernel: Option<File>,
    pub initrd: Option<File>,
    pub android_fstab: Option<File>,
    pub disks: Vec<DiskFile>,
    pub params: Option<String>,
    pub protected: bool,
    pub debug_config: DebugConfig,
    pub memory_mib: NonZeroU32,
    pub cpus: Option<NonZeroU32>,
    pub host_cpu_topology: bool,
    pub console_out_fd: Option<File>,
    pub console_in_fd: Option<File>,
    pub log_fd: Option<File>,
    pub ramdump: Option<File>,
    pub indirect_files: Vec<File>,
    pub platform_version: VersionReq,
    pub detect_hangup: bool,
    pub gdb_port: Option<NonZeroU16>,
    pub vfio_devices: Vec<VfioDevice>,
    pub dtbo: Option<File>,
    pub device_tree_overlay: Option<File>,
    pub display_config: Option<DisplayConfig>,
    pub input_device_options: Vec<InputDeviceOption>,
    pub hugepages: bool,
    pub tap: Option<File>,
    pub console_input_device: Option<String>,
    pub boost_uclamp: bool,
    pub gpu_config: Option<GpuConfig>,
    pub audio_config: Option<AudioConfig>,
    pub no_balloon: bool,
    pub usb_config: UsbConfig,
}

#[derive(Debug)]
pub struct AudioConfig {
    pub use_microphone: bool,
    pub use_speaker: bool,
}

impl AudioConfig {
    pub fn new(raw_config: &AudioConfigParcelable) -> Self {
        AudioConfig { use_microphone: raw_config.useMicrophone, use_speaker: raw_config.useSpeaker }
    }
}

#[derive(Debug)]
pub struct UsbConfig {
    pub controller: bool,
}

impl UsbConfig {
    pub fn new(raw_config: &UsbConfigParcelable) -> Result<UsbConfig> {
        Ok(UsbConfig { controller: raw_config.controller })
    }
}

#[derive(Debug)]
pub struct DisplayConfig {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
    pub horizontal_dpi: NonZeroU32,
    pub vertical_dpi: NonZeroU32,
    pub refresh_rate: NonZeroU32,
}

impl DisplayConfig {
    pub fn new(raw_config: &DisplayConfigParcelable) -> Result<DisplayConfig> {
        let width = try_into_non_zero_u32(raw_config.width)?;
        let height = try_into_non_zero_u32(raw_config.height)?;
        let horizontal_dpi = try_into_non_zero_u32(raw_config.horizontalDpi)?;
        let vertical_dpi = try_into_non_zero_u32(raw_config.verticalDpi)?;
        let refresh_rate = try_into_non_zero_u32(raw_config.refreshRate)?;
        Ok(DisplayConfig { width, height, horizontal_dpi, vertical_dpi, refresh_rate })
    }
}

#[derive(Debug)]
pub struct GpuConfig {
    pub backend: Option<String>,
    pub context_types: Option<Vec<String>>,
    pub pci_address: Option<String>,
    pub renderer_features: Option<String>,
    pub renderer_use_egl: Option<bool>,
    pub renderer_use_gles: Option<bool>,
    pub renderer_use_glx: Option<bool>,
    pub renderer_use_surfaceless: Option<bool>,
    pub renderer_use_vulkan: Option<bool>,
}

impl GpuConfig {
    pub fn new(raw_config: &GpuConfigParcelable) -> Result<GpuConfig> {
        Ok(GpuConfig {
            backend: raw_config.backend.clone(),
            context_types: raw_config.contextTypes.clone().map(|context_types| {
                context_types.iter().filter_map(|context_type| context_type.clone()).collect()
            }),
            pci_address: raw_config.pciAddress.clone(),
            renderer_features: raw_config.rendererFeatures.clone(),
            renderer_use_egl: Some(raw_config.rendererUseEgl),
            renderer_use_gles: Some(raw_config.rendererUseGles),
            renderer_use_glx: Some(raw_config.rendererUseGlx),
            renderer_use_surfaceless: Some(raw_config.rendererUseSurfaceless),
            renderer_use_vulkan: Some(raw_config.rendererUseVulkan),
        })
    }
}

fn try_into_non_zero_u32(value: i32) -> Result<NonZeroU32> {
    let u32_value = value.try_into()?;
    NonZeroU32::new(u32_value).ok_or_else(|| anyhow!("value should be greater than 0"))
}

#[derive(Debug)]
pub struct DiskFile {
    pub image: File,
    pub writable: bool,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum InputDeviceOption {
    EvDev(File),
    SingleTouch { file: File, width: u32, height: u32, name: Option<String> },
    Keyboard(File),
    Mouse(File),
    Switches(File),
    MultiTouchTrackpad { file: File, width: u32, height: u32, name: Option<String> },
    MultiTouch { file: File, width: u32, height: u32, name: Option<String> },
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PayloadState {
    Starting,
    Started,
    Ready,
    Finished,
    Hangup,
}

#[derive(Debug)]
pub enum VmState {
    NotStarted { config: Box<CrosvmConfig> },
    Running { child: Arc<SharedChild>, monitor_vm_exit_thread: Option<JoinHandle<()>> },
    Dead,
    Failed,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rss {
    pub vm: i64,
    pub crosvm: i64,
}

#[derive(Debug, Default)]
pub struct VmMetric {
    pub start_timestamp: Option<SystemTime>,
    pub cpu_guest_time: Option<i64>,
    pub rss: Option<Rss>,
}

impl VmState {
    fn start(&mut self, instance: Arc<VmInstance>) -> Result<(), Error> {
        let state = mem::replace(self, VmState::Failed);
        if let VmState::NotStarted { config } = state {
            let config = *config;
            let detect_hangup = config.detect_hangup;
            let vfio_devices = config.vfio_devices.clone();

            debug_trace(format!("virtmgr: before run_vm cid={}", instance.cid));
            eprintln!("virtmgr: before run_vm cid={}", instance.cid);
            let console_output = config
                .console_out_fd
                .as_ref()
                .and_then(|file| file.try_clone().ok());
            let console_pipes = instance.start_console_pipe(console_output);
            let (console_pipe_path, console_input_pipe_path) = match console_pipes {
                Some((out, inp)) => (Some(out), inp),
                None => (None, None),
            };
            let (child, keepalive) = run_vm(
                config,
                &instance.temporary_directory,
                &instance.crosvm_control_socket_path,
                console_pipe_path,
                console_input_pipe_path,
            )?;
            debug_trace(format!("virtmgr: after run_vm cid={} child={}", instance.cid, child.id()));
            eprintln!("virtmgr: after run_vm cid={} child={}", instance.cid, child.id());
            *instance.keepalive_indirect_files.lock().unwrap() = keepalive;
            let child = Arc::new(child);

            let instance_monitor_status = instance.clone();
            let child_monitor_status = child.clone();
            thread::spawn(move || {
                instance_monitor_status.clone().monitor_vm_status(child_monitor_status);
            });

            let child_clone = child.clone();
            let instance_clone = instance.clone();
            let monitor_vm_exit_thread = Some(thread::spawn(move || {
                instance_clone.monitor_vm_exit(child_clone, vfio_devices);
            }));

            if detect_hangup {
                let child_clone = child.clone();
                thread::spawn(move || {
                    instance.monitor_payload_hangup(child_clone);
                });
            }

            *self = VmState::Running { child, monitor_vm_exit_thread };
            Ok(())
        } else {
            *self = state;
            bail!("VM already started or failed")
        }
    }
}

#[derive(Debug)]
pub struct VmContext {
    #[allow(dead_code)]
    pub(crate) global_context: Strong<dyn IGlobalVmContext>,
    #[allow(dead_code)]
    vm_server: RpcServer,
}

impl VmContext {
    pub fn new(global_context: Strong<dyn IGlobalVmContext>, vm_server: RpcServer) -> VmContext {
        VmContext { global_context, vm_server }
    }
}

#[derive(Debug)]
pub struct VmInstance {
    pub vm_state: Mutex<VmState>,
    pub(crate) vm_context: VmContext,
    pub cid: Cid,
    crosvm_control_socket_path: PathBuf,
    pub name: String,
    pub protected: bool,
    pub temporary_directory: PathBuf,
    pub requester_uid: u32,
    pub requester_debug_pid: i32,
    pub callbacks: VirtualMachineCallbacks,
    pub vm_service: Mutex<Option<Strong<dyn IVirtualMachineService>>>,
    pub vm_metric: Mutex<VmMetric>,
    payload_state: Mutex<PayloadState>,
    payload_state_updated: Condvar,
    host_console_name: Mutex<Option<String>>,
    requester_uid_name: String,
    /// Keeps composite-disk partition `File` handles alive until the VM exits (paths are embedded in
    /// the composite image; crosvm opens by path).
    keepalive_indirect_files: Mutex<Vec<File>>,
    host_vsock_tcp_bridges: Mutex<HashMap<u16, Arc<AtomicBool>>>,
    #[cfg(windows)]
    console_output_buffer: Arc<Mutex<Vec<u8>>>,
    #[cfg(windows)]
    console_reader: Mutex<Option<JoinHandle<()>>>,
    #[cfg(windows)]
    console_input_pipe: Mutex<Option<OwnedHandle>>,
    #[cfg(windows)]
    console_input_pipe_path: Mutex<Option<String>>,
}

impl fmt::Display for VmInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let adj = if self.protected { "Protected" } else { "Non-protected" };
        write!(
            f,
            "{} virtual machine \"{}\" (owner: {}, cid: {})",
            adj, self.name, self.requester_uid_name, self.cid
        )
    }
}

impl VmInstance {
    pub fn new(
        config: CrosvmConfig,
        temporary_directory: PathBuf,
        requester_uid: u32,
        requester_debug_pid: i32,
        vm_context: VmContext,
    ) -> Result<VmInstance, Error> {
        validate_config(&config)?;
        validate_windows_host(&config)?;
        let cid = config.cid;
        let name = config.name.clone();
        let protected = config.protected;
        let requester_uid_name = format!("{}", requester_uid);
        let control_pipe = make_control_pipe_path(cid);
        let instance = VmInstance {
            vm_state: Mutex::new(VmState::NotStarted { config: Box::new(config) }),
            vm_context,
            cid,
            crosvm_control_socket_path: control_pipe,
            name,
            protected,
            temporary_directory,
            requester_uid,
            requester_debug_pid,
            callbacks: Default::default(),
            vm_service: Mutex::new(None),
            vm_metric: Mutex::new(Default::default()),
            payload_state: Mutex::new(PayloadState::Starting),
            payload_state_updated: Condvar::new(),
            host_console_name: Mutex::new(None),
            requester_uid_name,
            keepalive_indirect_files: Mutex::new(Vec::new()),
            host_vsock_tcp_bridges: Mutex::new(HashMap::new()),
            console_output_buffer: Arc::new(Mutex::new(Vec::new())),
            console_reader: Mutex::new(None),
            console_input_pipe: Mutex::new(None),
            console_input_pipe_path: Mutex::new(None),
        };
        info!("{} created", &instance);
        Ok(instance)
    }

    pub fn start(self: &Arc<Self>) -> Result<(), Error> {
        let mut vm_metric = self.vm_metric.lock().unwrap();
        vm_metric.start_timestamp = Some(SystemTime::now());
        debug_trace(format!("virtmgr: before VmInstance::start cid={}", self.cid));
        eprintln!("virtmgr: before VmInstance::start cid={}", self.cid);
        let ret = self.vm_state.lock().unwrap().start(self.clone());
        if ret.is_ok() {
            info!("{} started", &self);
        }
        debug_trace(format!(
            "virtmgr: after VmInstance::start cid={} ok={}",
            self.cid,
            ret.is_ok()
        ));
        eprintln!("virtmgr: after VmInstance::start cid={} ok={}", self.cid, ret.is_ok());
        ret.with_context(|| format!("{} failed to start", &self))
    }

    pub fn host_console_name(&self) -> Option<String> {
        self.host_console_name.lock().unwrap().clone()
    }

    pub fn remember_host_console_name(&self, host_console_name: &str) {
        *self.host_console_name.lock().unwrap() = Some(host_console_name.to_owned());
    }

    /// Create a named pipe for guest console output and start a reader thread.
    /// Returns the pipe path for use in crosvm serial arguments, or None if pipe creation fails.
    fn start_console_pipe(&self, output: Option<File>) -> Option<(String, Option<String>)> {
        // Output pipe: crosvm writes serial output, we read from it
        let (out_pipe_path, out_pipe_handle) = match create_console_named_pipe(self.cid) {
            Ok(v) => v,
            Err(e) => {
                warn!("Failed to create console named pipe: {e:#}; falling back to file-backed console");
                return None;
            }
        };

        let buf = self.console_output_buffer.clone();
        let reader = thread::Builder::new()
            .name(format!("virtmgr-console-reader-{}", self.cid))
            .spawn(move || {
                console_pipe_read_loop(out_pipe_handle, buf, output);
            });

        let out_path = match reader {
            Ok(handle) => {
                *self.console_reader.lock().unwrap() = Some(handle);
                info!("Console output pipe started: {out_pipe_path}");
                out_pipe_path
            }
            Err(e) => {
                warn!("Failed to spawn console reader thread: {e}; falling back to file-backed console");
                return None;
            }
        };

        // The output pipe is server-owned by virtmgr and crosvm connects as its writer.
        // Do not also create a named input pipe here: crosvm's dual-pipe console path owns
        // both server endpoints, so mixing the two ownership models races with ERROR_PIPE_BUSY.
        // Console input, when supplied, remains backed by `config.console_in_fd`.
        Some((out_path, None))
    }

    /// Read buffered console output, up to `max_len` bytes. Clears the buffer after reading.
    pub fn read_console(&self, max_len: usize) -> Vec<u8> {
        let mut buf = self.console_output_buffer.lock().unwrap();
        let len = buf.len().min(max_len);
        let data: Vec<u8> = buf.drain(..len).collect();
        data
    }

    /// Write data to guest console input via the bidirectional named pipe.
    /// Returns the number of bytes written.
    pub fn write_console(&self, data: &[u8]) -> std::io::Result<usize> {
        let pipe_guard = self.console_input_pipe.lock().unwrap();
        let Some(ref handle) = *pipe_guard else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "console input pipe not available",
            ));
        };

        let raw = handle.as_raw_handle() as _;
        let mut written = 0u32;
        let rc = unsafe {
            windows_sys::Win32::Storage::FileSystem::WriteFile(
                raw,
                data.as_ptr() as _,
                data.len() as u32,
                &mut written,
                ptr::null_mut(),
            )
        };
        if rc == 0 {
            Err(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "console input pipe write failed",
            ))
        } else {
            Ok(written as usize)
        }
    }

    fn monitor_vm_exit(&self, child: Arc<SharedChild>, _vfio_devices: Vec<VfioDevice>) {
        let result = child.wait();
        match &result {
            Err(e) => error!("Error waiting for crosvm({}) instance to die: {}", child.id(), e),
            Ok(status) => {
                debug_trace(format!(
                    "virtmgr: monitor_vm_exit status cid={} code={:?} success={}",
                    self.cid,
                    status.code(),
                    status.success()
                ));
                info!("crosvm({}) exited with status {}", child.id(), status);
                if let Some(exit_status_code) = status.code() {
                    if exit_status_code == CROSVM_WATCHDOG_REBOOT_STATUS {
                        info!("detected vcpu stall on crosvm");
                    }
                }
            }
        }

        let mut vm_state = self.vm_state.lock().unwrap();
        *vm_state = VmState::Dead;
        drop(vm_state);
        self.payload_state_updated.notify_all();
        info!("{} exited", &self);

        let mut failure_reason = String::new();
        let failure_path = self.temporary_directory.join("vm_failure_serial.txt");
        match File::open(&failure_path).and_then(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }) {
            Err(e) => debug!("No VM failure serial output ({}): {}", failure_path.display(), e),
            Ok(s) if !s.is_empty() => {
                info!("VM returned failure reason '{}'", &s);
                failure_reason = s;
            }
            Ok(_) => {}
        }
        let failure_reason =
            if failure_reason.is_empty() && self.payload_state() == PayloadState::Hangup {
                Cow::from("HANGUP")
            } else {
                Cow::from(failure_reason)
            };

        self.handle_ramdump().unwrap_or_else(|e| error!("Error handling ramdump: {}", e));

        let death_reason = death_reason(&result, failure_reason.as_ref());
        let exit_signal = exit_signal(&result);

        self.stop_host_vsock_tcp_bridges();
        debug_trace(format!(
            "virtmgr: monitor_vm_exit callback_on_died cid={} reason={:?}",
            self.cid, death_reason
        ));
        self.callbacks.callback_on_died(self.cid, death_reason);

        let vm_metric = self.vm_metric.lock().unwrap();
        write_vm_exited_stats_sync(
            self.requester_uid as i32,
            &self.name,
            death_reason,
            exit_signal,
            &vm_metric,
        );

        remove_temporary_files(&self.temporary_directory).unwrap_or_else(|e| {
            error!("Error removing temporary files from {:?}: {}", self.temporary_directory, e);
        });
    }

    fn monitor_payload_hangup(&self, child: Arc<SharedChild>) {
        debug!("Starting to monitor hangup for Microdroid({})", child.id());
        let deadline = Instant::now() + *BOOT_HANGUP_TIMEOUT;
        loop {
            {
                let state = self.payload_state.lock().unwrap();
                if *state >= PayloadState::Started {
                    return;
                }
            }
            if matches!(*self.vm_state.lock().unwrap(), VmState::Dead | VmState::Failed) {
                return;
            }
            let now = Instant::now();
            if now >= deadline {
                break;
            }
            let timeout = deadline.saturating_duration_since(now);
            let (state, wait_result) = self
                .payload_state_updated
                .wait_timeout(self.payload_state.lock().unwrap(), timeout)
                .unwrap();
            drop(state);
            if wait_result.timed_out() {
                break;
            }
        }
        let child_still_running = child.try_wait().ok() == Some(None);
        if child_still_running {
            error!(
                "Microdroid({}) failed to start payload within {} secs timeout. Shutting down.",
                child.id(),
                BOOT_HANGUP_TIMEOUT.as_secs()
            );
            self.update_payload_state(PayloadState::Hangup).unwrap();
            if let Err(e) = self.kill() {
                error!("Error stopping timed-out VM with CID {}: {:?}", child.id(), e);
            }
        }
    }

    fn monitor_vm_status(&self, child: Arc<SharedChild>) {
        let _pid = child.id();
        loop {
            {
                let vm_state = &*self.vm_state.lock().unwrap();
                if let VmState::Dead = vm_state {
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    pub fn payload_state(&self) -> PayloadState {
        *self.payload_state.lock().unwrap()
    }

    pub fn update_payload_state(&self, new_state: PayloadState) -> Result<(), Error> {
        let mut state_locked = self.payload_state.lock().unwrap();
        if new_state > *state_locked {
            *state_locked = new_state;
            self.payload_state_updated.notify_all();
            Ok(())
        } else {
            bail!("Invalid payload state transition from {:?} to {:?}", *state_locked, new_state)
        }
    }

    pub fn kill(&self) -> Result<(), Error> {
        self.stop_host_vsock_tcp_bridges();
        let monitor_vm_exit_thread = {
            let vm_state = &mut *self.vm_state.lock().unwrap();
            if let VmState::Running { child, monitor_vm_exit_thread } = vm_state {
                let id = child.id();
                debug!("Killing crosvm({})", id);
                child.kill().with_context(|| format!("Error killing crosvm({id}) instance"))?;
                monitor_vm_exit_thread.take()
            } else {
                bail!("VM is not running")
            }
        };

        monitor_vm_exit_thread.map(JoinHandle::join);

        self.vm_context.vm_server.shutdown()?;

        Ok(())
    }

    pub fn get_memory_balloon(&self) -> Result<u64, Error> {
        let request = VmRequest::BalloonCommand(BalloonControlCommand::Stats {});
        let result =
            match vm_control::client::handle_request(&request, &self.crosvm_control_socket_path) {
                Ok(VmResponse::BalloonStats { stats: _, balloon_actual }) => balloon_actual,
                Ok(VmResponse::Err(e)) => {
                    if e.errno() != libc::ENOTSUP {
                        bail!("Errno return when requesting balloon stats: {}", e.errno())
                    }
                    0
                }
                e => bail!("Error requesting balloon stats: {:?}", e),
            };
        Ok(result)
    }

    pub fn set_memory_balloon(&self, num_bytes: u64) -> Result<(), Error> {
        let command = BalloonControlCommand::Adjust { num_bytes, wait_for_success: false };
        vm_control::client::handle_request(
            &VmRequest::BalloonCommand(command),
            &self.crosvm_control_socket_path,
        )
        .map_err(|e| anyhow!("Error sending balloon adjustment: {}", e))?;
        Ok(())
    }

    fn handle_ramdump(&self) -> Result<(), Error> {
        let ramdump_path = self.temporary_directory.join("ramdump");
        if !ramdump_path.as_path().try_exists()? {
            return Ok(());
        }
        if std::fs::metadata(&ramdump_path)?.len() > 0 {
            info!(
                "Ramdump at {:?} (tombstoned upload is not available on this host)",
                ramdump_path
            );
        }
        Ok(())
    }

    pub fn suspend(&self) -> Result<(), Error> {
        match vm_control::client::handle_request(
            &VmRequest::SuspendVcpus,
            &self.crosvm_control_socket_path,
        ) {
            Ok(VmResponse::Ok) => Ok(()),
            e => bail!("Failed to suspend VM: {e:?}"),
        }
    }

    pub fn resume(&self) -> Result<(), Error> {
        match vm_control::client::handle_request(
            &VmRequest::ResumeVcpus,
            &self.crosvm_control_socket_path,
        ) {
            Ok(VmResponse::Ok) => Ok(()),
            e => bail!("Failed to resume: {e:?}"),
        }
    }

    pub fn prepare_vsock_connection(&self, port: u32) -> Result<(), Error> {
        prepare_vsock_connection_path(&self.crosvm_control_socket_path, port)
    }

    pub fn start_host_vsock_tcp_bridge(
        &self,
        host_port: u16,
        guest_port: u32,
    ) -> Result<(), Error> {
        {
            let bridges = self.host_vsock_tcp_bridges.lock().unwrap();
            if let Some(active) = bridges.get(&host_port) {
                if active.load(Ordering::Relaxed) {
                    debug_trace(format!(
                        "virtmgr: host bridge already active cid={} host_port={} guest_port={}",
                        self.cid, host_port, guest_port
                    ));
                    return Ok(());
                }
            }
        }

        let listener = TcpListener::bind(("127.0.0.1", host_port))
            .with_context(|| format!("failed to bind localhost:{host_port}"))?;
        listener
            .set_nonblocking(true)
            .with_context(|| format!("failed to set nonblocking localhost:{host_port}"))?;

        let running = Arc::new(AtomicBool::new(true));
        self.host_vsock_tcp_bridges.lock().unwrap().insert(host_port, Arc::clone(&running));

        let cid = self.cid;
        let control_socket = self.crosvm_control_socket_path.clone();
        thread::Builder::new()
            .name(format!("virtmgr-host-vsock-bridge-{host_port}"))
            .spawn(move || {
                debug_trace(format!(
                    "virtmgr: host bridge listening cid={} host_port={} guest_port={}",
                    cid, host_port, guest_port
                ));
                while running.load(Ordering::Acquire) {
                    match listener.accept() {
                        Ok((tcp, remote)) => {
                            // The listener must stay nonblocking so it can observe shutdown, while
                            // each accepted data stream is consumed by blocking io::copy. Set the
                            // child stream explicitly instead of relying on platform inheritance.
                            if let Err(err) = tcp.set_nonblocking(false) {
                                error!(
                                    "Failed to make host bridge stream blocking for cid={} host_port={} guest_port={}: {}",
                                    cid, host_port, guest_port, err
                                );
                                continue;
                            }
                            debug_trace(format!(
                                "virtmgr: host bridge accepted cid={} host_port={} guest_port={} remote={}",
                                cid, host_port, guest_port, remote
                            ));
                            let control_socket = control_socket.clone();
                            thread::spawn(move || {
                                if let Err(err) =
                                    bridge_tcp_client_to_guest_vsock(cid, &control_socket, guest_port, tcp)
                                {
                                    debug_trace(format!(
                                        "virtmgr: host bridge client failed cid={} host_port={} guest_port={} error={:#}",
                                        cid, host_port, guest_port, err
                                    ));
                                }
                            });
                        }
                        Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                            thread::sleep(Duration::from_millis(100));
                        }
                        Err(err) => {
                            debug_trace(format!(
                                "virtmgr: host bridge listener error cid={} host_port={} guest_port={} error={}",
                                cid, host_port, guest_port, err
                            ));
                            break;
                        }
                    }
                }
                debug_trace(format!(
                    "virtmgr: host bridge stopped cid={} host_port={} guest_port={}",
                    cid, host_port, guest_port
                ));
            })
            .with_context(|| format!("failed to spawn host bridge thread for localhost:{host_port}"))?;
        Ok(())
    }

    fn stop_host_vsock_tcp_bridges(&self) {
        let bridges = std::mem::take(&mut *self.host_vsock_tcp_bridges.lock().unwrap());
        for active in bridges.into_values() {
            active.store(false, Ordering::Release);
        }
    }
}

fn prepare_vsock_connection_path(control_socket_path: &Path, port: u32) -> Result<(), Error> {
    match vm_control::client::handle_request(&VmRequest::ConnectVsock { port }, control_socket_path)
    {
        Ok(VmResponse::Ok) => Ok(()),
        Ok(VmResponse::Err(e)) => {
            bail!("Failed to prepare vsock connection: errno {}", e.errno())
        }
        other => bail!("Unexpected vsock preparation response: {other:?}"),
    }
}

fn bridge_tcp_client_to_guest_vsock(
    cid: Cid,
    control_socket_path: &Path,
    guest_port: u32,
    tcp: TcpStream,
) -> Result<(), Error> {
    prepare_vsock_connection_path(control_socket_path, guest_port)?;
    let handle = crate::vsock_transport::connect(cid, guest_port).with_context(|| {
        format!("failed to connect guest vsock pipe for cid={cid} port={guest_port}")
    })?;
    let guest = File::from(handle);
    crate::bridge::bridge_connection(tcp, guest)
        .map_err(|e| anyhow::anyhow!("bridge_connection failed: {e}"))
}

fn make_control_pipe_path(cid: Cid) -> PathBuf {
    let id = Uuid::new_v4();
    PathBuf::from(format!(r"\\.\pipe\virtmgr_crosvm_{cid}_{id}"))
}

fn validate_config(config: &CrosvmConfig) -> Result<(), Error> {
    if config.bootloader.is_none() && config.kernel.is_none() {
        bail!("VM must have either a bootloader or a kernel image.");
    }
    if config.bootloader.is_some() && (config.kernel.is_some() || config.initrd.is_some()) {
        bail!("Can't have both bootloader and kernel/initrd image.");
    }
    let version = Version::parse(CROSVM_PLATFORM_VERSION).unwrap();
    if !config.platform_version.matches(&version) {
        bail!(
            "Incompatible platform version. The config is compatible with platform version(s) \
              {}, but the actual platform version is {}",
            config.platform_version,
            version
        );
    }
    Ok(())
}

fn validate_windows_host(config: &CrosvmConfig) -> Result<(), Error> {
    if !config.vfio_devices.is_empty() {
        bail!(
            "VFIO passthrough is only present in crosvm Linux/Android `run` command lines; \
             the Windows crosvm binary does not accept `--vfio` / platform sysfs paths"
        );
    }
    if config.tap.is_some() {
        bail!(
            "Host TAP (`--net` / tap-fd) is only built for Unix crosvm with the `net` feature; \
             Windows `crosvm run` does not expose the same networking flags"
        );
    }
    if config.boost_uclamp {
        bail!("--boost-uclamp is Linux/Android-only in crosvm and is not passed on Windows");
    }
    Ok(())
}

fn run_vm(
    mut config: CrosvmConfig,
    temporary_directory: &Path,
    crosvm_control_socket_path: &Path,
    console_pipe_path: Option<String>,
    console_input_pipe_path: Option<String>,
) -> Result<(SharedChild, Vec<File>), Error> {
    validate_config(&config)?;
    validate_windows_host(&config)?;

    let binary = crosvm_binary();
    debug_trace(format!("virtmgr: run_vm binary={:?} cid={}", binary, config.cid));
    eprintln!("virtmgr: run_vm binary={:?} cid={}", binary, config.cid);
    let mut command = Command::new(&binary);
    let log_level = std::env::var("VIRTMGR_CROSVM_LOG_LEVEL")
        .unwrap_or_else(|_| "info,disk=warn".to_owned());
    command
        .arg("--extended-status")
        .arg("--log-level")
        .arg(log_level)
        .arg("run")
        .arg("--disable-sandbox")
        .arg("--cid")
        .arg(config.cid.to_string());

    if !config.no_balloon {
        command.arg("--balloon-page-reporting");
    } else {
        command.arg("--no-balloon");
    }

    if !config.usb_config.controller {
        command.arg("--no-usb");
    }

    let mut memory_mib = config.memory_mib;

    if config.protected {
        match rustutils::system_properties::read(SYSPROP_CUSTOM_PVMFW_PATH)? {
            Some(pvmfw_path) if !pvmfw_path.is_empty() => {
                command.arg("--protected-vm-with-firmware").arg(pvmfw_path);
            }
            _ => {
                command.arg("--protected-vm");
            }
        }

        let virtio_pci_device_count = 4 + config.disks.len();
        let swiotlb_size_mib = 2 * virtio_pci_device_count as u32;
        command.arg("--swiotlb").arg(swiotlb_size_mib.to_string());
        memory_mib = memory_mib.saturating_add(swiotlb_size_mib);
        if config.ramdump.is_some() {
            let ramdump_reserve = RAMDUMP_RESERVED_MIB + swiotlb_size_mib;
            command.arg("--params").arg(format!("crashkernel={ramdump_reserve}M"));
        }
    } else if config.ramdump.is_some() {
        command.arg("--params").arg(format!("crashkernel={RAMDUMP_RESERVED_MIB}M"));
    }

    if let Ok(entries) = std::fs::read_dir(temporary_directory) {
        let listing = entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        debug_trace(format!(
            "virtmgr: temp dir cid={} path={} entries={:?}",
            config.cid,
            temporary_directory.display(),
            listing
        ));
    }

    if config.debug_config.debug_level == DebugLevel::NONE
        && config.debug_config.should_prepare_console_output()
    {
        command.arg("--params").arg("printk.devkmsg=on");
        command.arg("--params").arg("console=hvc0");
    }

    command.arg("--mem").arg(memory_mib.to_string());

    if config.host_cpu_topology {
        if let Some(cpus) = get_num_cpus() {
            command.arg("--cpus").arg(cpus.to_string());
        } else {
            bail!("Could not determine the number of CPUs in the system");
        }
    } else if let Some(cpus) = config.cpus {
        command.arg("--cpus").arg(cpus.to_string());
    }

    if let Some(gdb_port) = config.gdb_port {
        command.arg("--gdb").arg(gdb_port.to_string());
    }

    // If named pipes were created for the console, use them instead of file paths.
    // Output pipe enables real-time console reading via `read_console()`.
    // Input pipe enables keyboard input via `write_console()` (Phase B).
    let console_out_arg = if let Some(ref pipe_path) = console_pipe_path {
        format!("type=file,path={pipe_path}")
    } else {
        format_serial_out_arg(config.console_out_fd.take())?
    };
    let console_in_arg = if let Some(ref input_pipe_path) = console_input_pipe_path {
        format!(",input={input_pipe_path}")
    } else {
        config
            .console_in_fd
            .take()
            .map(|fd| add_path(fd).map(|p| format!(",input={p}")))
            .transpose()?
            .unwrap_or_default()
    };
    let log_arg = format_serial_out_arg(config.log_fd.take())?;

    let failure_path = temporary_directory.join("vm_failure_serial.txt");
    OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(true)
        .open(&failure_path)
        .with_context(|| format!("failed to create {}", failure_path.display()))?;
    let failure_serial_path = failure_path.to_string_lossy().into_owned();

    let ramdump_arg = format_serial_out_arg(config.ramdump.take())?;
    let console_input_device = config.console_input_device.as_deref().unwrap_or(CONSOLE_HVC0);
    match console_input_device {
        CONSOLE_HVC0 | CONSOLE_TTYS0 => {}
        _ => bail!("Unsupported serial device {console_input_device}"),
    }

    let (serial1_arg, console1_arg, log3_arg) = if guest_console_capture_enabled() {
        let serial1_path = temporary_directory.join("guest-serial-num1.txt");
        let console1_path = temporary_directory.join("guest-virtio-console1.txt");
        let log3_path = temporary_directory.join("guest-virtio-console3.txt");
        OpenOptions::new().create(true).write(true).truncate(true).open(&serial1_path)?;
        OpenOptions::new().create(true).write(true).truncate(true).open(&console1_path)?;
        OpenOptions::new().create(true).write(true).truncate(true).open(&log3_path)?;
        (
            format!("type=file,path={}", serial1_path.to_string_lossy()),
            format!("type=file,path={}", console1_path.to_string_lossy()),
            format!("type=file,path={}", log3_path.to_string_lossy()),
        )
    } else if console_input_device == CONSOLE_TTYS0 {
        (console_out_arg.clone(), "type=sink".to_owned(), log_arg.clone())
    } else {
        ("type=sink".to_owned(), console_out_arg.clone(), log_arg.clone())
    };

    command.arg(format!(
        "--serial={}{},hardware=serial,num=1",
        &serial1_arg,
        if console_input_device == CONSOLE_TTYS0 { &console_in_arg } else { "" }
    ));
    command.arg(format!("--serial=type=file,path={},hardware=serial,num=2", failure_serial_path));
    command.arg(format!(
        "--serial={}{},hardware=virtio-console,num=1",
        &console1_arg,
        if console_input_device == CONSOLE_HVC0 { &console_in_arg } else { "" }
    ));
    command.arg(format!("--serial={},hardware=virtio-console,num=2", &ramdump_arg));
    command.arg(format!("--serial={},hardware=virtio-console,num=3", &log3_arg));

    if let Some(bootloader) = config.bootloader.take() {
        command.arg("--bios").arg(add_path(bootloader)?);
    }

    if let Some(initrd) = config.initrd.take() {
        command.arg("--initrd").arg(add_path(initrd)?);
    }

    if let Some(android_fstab) = config.android_fstab.take() {
        command.arg("--android-fstab").arg(add_path(android_fstab)?);
    }

    if let Some(params) = &config.params {
        command.arg("--params").arg(params);
    }

    for disk in config.disks {
        if let Ok(path) =
            disk.image.try_clone().and_then(|f| file_path_for_crosvm(&f).map_err(io::Error::other))
        {
            debug_trace(format!(
                "virtmgr: disk path cid={} path={} exists={}",
                config.cid,
                path.display(),
                path.exists()
            ));
        }
        command.arg("--block").arg(format!(
            "path={},ro={},lock=false,sparse=false",
            add_path(disk.image)?,
            !disk.writable,
        ));
    }

    if let Some(kernel) = config.kernel.take() {
        command.arg(add_path(kernel)?);
    }

    command.arg("--socket").arg(crosvm_control_socket_path);

    if crosvm_stdio_capture_enabled() {
        let stdout_path = temporary_directory.join("crosvm-stdout.txt");
        let stderr_path = temporary_directory.join("crosvm-stderr.txt");
        command
            .stdout(OpenOptions::new().create(true).write(true).truncate(true).open(&stdout_path)?);
        command
            .stderr(OpenOptions::new().create(true).write(true).truncate(true).open(&stderr_path)?);
    }

    if let Some(dt_overlay) = config.device_tree_overlay.take() {
        command.arg("--device-tree-overlay").arg(add_path(dt_overlay)?);
    }

    if cfg!(paravirtualized_devices) {
        if let Some(gpu_config) = &config.gpu_config {
            let mut gpu_args = Vec::new();
            if let Some(backend) = &gpu_config.backend {
                gpu_args.push(format!("backend={}", backend));
            }
            if let Some(context_types) = &gpu_config.context_types {
                gpu_args.push(format!("context-types={}", context_types.join(":")));
            }
            if let Some(pci_address) = &gpu_config.pci_address {
                gpu_args.push(format!("pci-address={}", pci_address));
            }
            if let Some(renderer_features) = &gpu_config.renderer_features {
                gpu_args.push(format!("renderer-features={}", renderer_features));
            }
            if gpu_config.renderer_use_egl.unwrap_or(false) {
                gpu_args.push("egl=true".to_string());
            }
            if gpu_config.renderer_use_gles.unwrap_or(false) {
                gpu_args.push("gles=true".to_string());
            }
            if gpu_config.renderer_use_glx.unwrap_or(false) {
                gpu_args.push("glx=true".to_string());
            }
            if gpu_config.renderer_use_surfaceless.unwrap_or(false) {
                gpu_args.push("surfaceless=true".to_string());
            }
            if gpu_config.renderer_use_vulkan.unwrap_or(false) {
                gpu_args.push("vulkan=true".to_string());
            }
            command.arg(format!("--gpu={}", gpu_args.join(",")));
        }
        if let Some(display_config) = &config.display_config {
            command
                .arg(format!(
                    "--gpu-display=mode=windowed[{},{}],dpi=[{},{}],refresh-rate={}",
                    display_config.width,
                    display_config.height,
                    display_config.horizontal_dpi,
                    display_config.vertical_dpi,
                    display_config.refresh_rate
                ))
                .arg(format!("--android-display-service={}", config.name));
        }
    }

    if cfg!(paravirtualized_devices) {
        for input_device_option in config.input_device_options.drain(..) {
            command.arg("--input");
            command.arg(windows_input_arg(input_device_option)?);
        }
    }

    if config.hugepages {
        command.arg("--hugepages");
    }

    let _ = config.audio_config.take();
    let _ = config.gpu_config.take();
    let _ = config.display_config.take();

    let _ = config.dtbo.take();

    let keepalive = std::mem::take(&mut config.indirect_files);

    info!("Running crosvm with args: {:?}", command.get_args().collect::<Vec<_>>());
    debug_trace(format!(
        "virtmgr: run_vm args cid={} args={:?}",
        config.cid,
        command.get_args().collect::<Vec<_>>()
    ));

    debug_trace(format!("virtmgr: before SharedChild::spawn cid={}", config.cid));
    eprintln!("virtmgr: before SharedChild::spawn cid={}", config.cid);
    let result = SharedChild::spawn(&mut command)?;
    debug_trace(format!(
        "virtmgr: after SharedChild::spawn cid={} pid={}",
        config.cid,
        result.id()
    ));
    eprintln!("virtmgr: after SharedChild::spawn cid={} pid={}", config.cid, result.id());
    debug!("Spawned crosvm({}).", result.id());
    Ok((result, keepalive))
}

fn death_reason(
    result: &Result<std::process::ExitStatus, io::Error>,
    mut failure_reason: &str,
) -> DeathReason {
    if let Some((reason, info)) = failure_reason.split_once('|') {
        error!("Failure info: {info}");
        failure_reason = reason;
    }
    if let Ok(status) = result {
        match failure_reason {
            "PVM_FIRMWARE_PUBLIC_KEY_MISMATCH" => {
                return DeathReason::PVM_FIRMWARE_PUBLIC_KEY_MISMATCH
            }
            "PVM_FIRMWARE_INSTANCE_IMAGE_CHANGED" => {
                return DeathReason::PVM_FIRMWARE_INSTANCE_IMAGE_CHANGED
            }
            "MICRODROID_FAILED_TO_CONNECT_TO_VIRTUALIZATION_SERVICE" => {
                return DeathReason::MICRODROID_FAILED_TO_CONNECT_TO_VIRTUALIZATION_SERVICE
            }
            "MICRODROID_PAYLOAD_HAS_CHANGED" => return DeathReason::MICRODROID_PAYLOAD_HAS_CHANGED,
            "MICRODROID_PAYLOAD_VERIFICATION_FAILED" => {
                return DeathReason::MICRODROID_PAYLOAD_VERIFICATION_FAILED
            }
            "MICRODROID_INVALID_PAYLOAD_CONFIG" => {
                return DeathReason::MICRODROID_INVALID_PAYLOAD_CONFIG
            }
            "MICRODROID_UNKNOWN_RUNTIME_ERROR" => {
                return DeathReason::MICRODROID_UNKNOWN_RUNTIME_ERROR
            }
            "HANGUP" => return DeathReason::HANGUP,
            _ => {}
        }
        match status.code() {
            None => DeathReason::KILLED,
            Some(0) => DeathReason::SHUTDOWN,
            Some(CROSVM_START_ERROR_STATUS) => DeathReason::START_FAILED,
            Some(CROSVM_REBOOT_STATUS) => DeathReason::REBOOT,
            Some(CROSVM_CRASH_STATUS) => DeathReason::CRASH,
            Some(CROSVM_WATCHDOG_REBOOT_STATUS) => DeathReason::WATCHDOG_REBOOT,
            Some(_) => DeathReason::UNKNOWN,
        }
    } else {
        DeathReason::INFRASTRUCTURE_ERROR
    }
}

fn exit_signal(_result: &Result<std::process::ExitStatus, io::Error>) -> Option<i32> {
    // Windows `ExitStatus` has no signal semantics (unlike Unix).
    None
}
