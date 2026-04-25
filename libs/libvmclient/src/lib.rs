// Copyright 2022, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.

//! Client library for VirtualizationService.

mod death_reason;
mod error_code;
mod errors;
mod sync;

#[cfg(unix)]
mod spawn_unix;
#[cfg(windows)]
mod spawn_windows;

pub use crate::death_reason::DeathReason;
pub use crate::error_code::ErrorCode;
pub use crate::errors::VmWaitError;
use crate::sync::Monitor;
use android_system_virtualizationcommon::aidl::android::system::virtualizationcommon::{
    DeathReason::DeathReason as AidlDeathReason, ErrorCode::ErrorCode as AidlErrorCode,
};
use android_system_virtualizationservice::{
    aidl::android::system::virtualizationservice::{
        IVirtualMachine::IVirtualMachine,
        IVirtualMachineCallback::{BnVirtualMachineCallback, IVirtualMachineCallback},
        IVirtualizationService::IVirtualizationService,
        VirtualMachineConfig::VirtualMachineConfig,
        VirtualMachineState::VirtualMachineState,
    },
    binder::{
        BinderFeatures, DeathRecipient, FromIBinder, IBinder, Interface, ParcelFileDescriptor,
        Result as BinderResult, StatusCode, Strong,
    },
};
use log::warn;
use rpcbinder::{FileDescriptorTransportMode, RpcSession};
use std::ffi::{c_char, c_int, c_void, CString};
use std::io;
use std::io::Read;
use std::io::Write;
#[cfg(windows)]
use std::net::{Shutdown, TcpListener, TcpStream};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, OwnedHandle, RawHandle};
#[cfg(windows)]
use std::sync::OnceLock;
#[cfg(windows)]
use std::thread;
use std::{
    fmt::{self, Debug, Formatter},
    fs::{File, OpenOptions},
    sync::Arc,
    time::Duration,
};
#[cfg(windows)]
use winapi::shared::minwindef::MAKEWORD;
#[cfg(windows)]
use winapi::shared::ntdef::HANDLE;
#[cfg(windows)]
use winapi::um::fileapi::{ReadFile, WriteFile};
#[cfg(windows)]
use winapi::um::handleapi::DuplicateHandle;
#[cfg(windows)]
use winapi::um::processthreadsapi::{GetCurrentProcess, GetExitCodeProcess, TerminateProcess};
#[cfg(windows)]
use winapi::um::winnt::DUPLICATE_SAME_ACCESS;
#[cfg(windows)]
use winapi::um::winsock2::{WSAStartup, WSADATA};

#[cfg(unix)]
use std::os::fd::AsFd;
#[cfg(unix)]
use std::os::fd::FromRawFd;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, IntoRawFd, OwnedFd};
#[cfg(unix)]
use std::os::unix::net::UnixStream;

const EARLY_VIRTMGR_PATH: &str = "/apex/com.android.virt/bin/early_virtmgr";
const VIRTMGR_PATH: &str = "/apex/com.android.virt/bin/virtmgr";
#[cfg(windows)]
const VIRTMGR_PATH_WIN: &str = "virtmgr.exe";
#[cfg(windows)]
const EARLY_VIRTMGR_PATH_WIN: &str = "early_virtmgr.exe";

#[cfg(not(windows))]
const VIRTMGR_THREADS: usize = 2;
#[cfg(windows)]
const VIRTMGR_THREADS: usize = 2;
#[cfg(windows)]
const HOST_RPC_CID: u32 = 1;
#[cfg(windows)]
const VMCLIENT_TRACE_FILE: &str = "VMCLIENT_TRACE_FILE";
#[cfg(windows)]
const STILL_ACTIVE_EXIT_CODE: u32 = 259;

#[cfg(windows)]
fn debug_trace(message: impl AsRef<str>) {
    let Ok(path) = std::env::var(VMCLIENT_TRACE_FILE) else {
        return;
    };

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        use std::io::Write;
        let _ = writeln!(file, "{}", message.as_ref());
    }
}

#[cfg(not(windows))]
fn debug_trace(_message: impl AsRef<str>) {}

#[cfg(windows)]
fn ensure_winsock_init() -> io::Result<()> {
    static INIT: OnceLock<io::Result<()>> = OnceLock::new();
    INIT.get_or_init(|| {
        let mut data: WSADATA = unsafe { std::mem::zeroed() };
        let rc = unsafe { WSAStartup(MAKEWORD(2, 2), &mut data) };
        if rc == 0 {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, format!("WSAStartup failed with code {rc}")))
        }
    })
    .as_ref()
    .map(|_| ())
    .map_err(|e| io::Error::new(e.kind(), e.to_string()))
}

/// Converts [`ParcelFileDescriptor`] from the Binder API `IVirtualMachine::connectVsock` into a raw
/// fd for [`RpcSession::setup_preconnected_client`].
///
/// Host transport (must match virtmgr `connectVsock`):
/// - **Linux / Android**: guest vsock → socket fd (`into_raw_fd`).
/// - **Windows**: named pipe → CRT fd via `open_osfhandle`.
/// - **macOS**: UDS client to `/tmp/binder_rpc_vsock_{cid}_{port}.sock` (see `macos_uds_vsock_path`).
fn pfd_from_connect_vsock_for_rpc(pfd: ParcelFileDescriptor) -> Option<libc::c_int> {
    #[cfg(windows)]
    {
        let h = pfd.into_raw_handle();
        let fd = unsafe { libc::open_osfhandle(h as isize, libc::O_RDWR | libc::O_BINARY) };
        if fd == -1 {
            warn!(
                "open_osfhandle for connectVsock ParcelFileDescriptor failed: {}",
                io::Error::last_os_error()
            );
            None
        } else {
            Some(fd)
        }
    }
    #[cfg(unix)]
    {
        Some(pfd.into_raw_fd())
    }
}

#[cfg(windows)]
fn fd_from_connect_vsock_pfd(pfd: ParcelFileDescriptor) -> libc::c_int {
    let handle = pfd.into_raw_handle();
    let fd = unsafe { libc::open_osfhandle(handle as isize, libc::O_RDWR | libc::O_BINARY) };
    if fd == -1 {
        panic!(
            "open_osfhandle for connectVsock ParcelFileDescriptor failed: {}",
            io::Error::last_os_error()
        );
    }
    fd
}

#[cfg(windows)]
fn handle_from_connect_vsock_pfd(pfd: ParcelFileDescriptor) -> OwnedHandle {
    unsafe { OwnedHandle::from_raw_handle(pfd.into_raw_handle()) }
}

#[cfg(windows)]
fn duplicate_owned_handle(handle: &OwnedHandle) -> io::Result<OwnedHandle> {
    let mut duplicated: HANDLE = std::ptr::null_mut();
    let ok = unsafe {
        DuplicateHandle(
            GetCurrentProcess(),
            handle.as_raw_handle() as HANDLE,
            GetCurrentProcess(),
            &mut duplicated,
            0,
            0,
            DUPLICATE_SAME_ACCESS,
        )
    };
    if ok == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(unsafe { OwnedHandle::from_raw_handle(duplicated as RawHandle) })
    }
}

#[cfg(windows)]
fn write_handle_all(handle: &OwnedHandle, mut buf: &[u8]) -> io::Result<()> {
    while !buf.is_empty() {
        let chunk_len = buf.len().min(u32::MAX as usize);
        let mut written = 0u32;
        let ok = unsafe {
            WriteFile(
                handle.as_raw_handle() as HANDLE,
                buf.as_ptr().cast(),
                chunk_len as u32,
                &mut written,
                std::ptr::null_mut(),
            )
        };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        if written == 0 {
            return Err(io::Error::new(io::ErrorKind::WriteZero, "named pipe write returned 0"));
        }
        buf = &buf[written as usize..];
    }
    Ok(())
}

#[cfg(windows)]
fn read_handle_some(handle: &OwnedHandle, buf: &mut [u8]) -> io::Result<usize> {
    let chunk_len = buf.len().min(u32::MAX as usize);
    let mut read = 0u32;
    let ok = unsafe {
        ReadFile(
            handle.as_raw_handle() as HANDLE,
            buf.as_mut_ptr().cast(),
            chunk_len as u32,
            &mut read,
            std::ptr::null_mut(),
        )
    };
    if ok == 0 {
        let err = io::Error::last_os_error();
        match err.raw_os_error() {
            Some(109) | Some(232) => Ok(0),
            _ => Err(err),
        }
    } else {
        Ok(read as usize)
    }
}

#[cfg(windows)]
fn bridge_vsock_handle_to_tcp(guest: OwnedHandle, tcp: TcpStream) {
    let tcp_read = match tcp.try_clone() {
        Ok(stream) => stream,
        Err(err) => {
            warn!("failed to clone tcp stream: {}", err);
            return;
        }
    };
    let guest_write = match duplicate_owned_handle(&guest) {
        Ok(handle) => handle,
        Err(err) => {
            warn!("failed to duplicate guest vsock handle: {}", err);
            let _ = tcp.shutdown(Shutdown::Both);
            return;
        }
    };

    let uplink = thread::spawn(move || {
        let mut tcp_read = tcp_read;
        let mut total = 0u64;
        let mut buf = [0u8; 4096];
        loop {
            match tcp_read.read(&mut buf) {
                Ok(0) => {
                    eprintln!("vm bridge: tcp->guest reached EOF after {} bytes", total);
                    break;
                }
                Ok(n) => {
                    total += n as u64;
                    eprintln!("vm bridge: tcp->guest read {} bytes", n);
                    match write_handle_all(&guest_write, &buf[..n]) {
                        Ok(()) => {
                            eprintln!("vm bridge: tcp->guest wrote {} bytes", n);
                        }
                        Err(err) => {
                            eprintln!(
                                "vm bridge: tcp->guest write failed after {} bytes: {}",
                                total, err
                            );
                            return;
                        }
                    }
                }
                Err(err) => {
                    eprintln!("vm bridge: tcp->guest read failed after {} bytes: {}", total, err);
                    break;
                }
            }
        }
    });

    let mut tcp_write = tcp;
    let mut total = 0u64;
    let mut buf = [0u8; 4096];
    loop {
        match read_handle_some(&guest, &mut buf) {
            Ok(0) => {
                eprintln!("vm bridge: guest->tcp reached EOF after {} bytes", total);
                break;
            }
            Ok(n) => {
                total += n as u64;
                eprintln!("vm bridge: guest->tcp read {} bytes", n);
                if let Err(err) = tcp_write.write_all(&buf[..n]) {
                    eprintln!("vm bridge: guest->tcp write failed after {} bytes: {}", total, err);
                    break;
                }
            }
            Err(err) => {
                eprintln!("vm bridge: guest->tcp read failed after {} bytes: {}", total, err);
                break;
            }
        }
    }
    let _ = uplink.join();
    let _ = tcp_write.shutdown(Shutdown::Both);
}

#[cfg(windows)]
fn bridge_vsock_pfd_to_tcp(pfd: ParcelFileDescriptor, tcp: TcpStream) {
    let guest = handle_from_connect_vsock_pfd(pfd);
    bridge_vsock_handle_to_tcp(guest, tcp);
}

#[cfg(unix)]
fn file_from_connect_vsock_pfd(pfd: ParcelFileDescriptor) -> File {
    let fd = pfd.into_raw_fd();
    unsafe { File::from_raw_fd(fd) }
}

/// Running virtmgr connection: Unix domain bootstrap fd (Unix) or RPC vsock port (Windows).
pub struct VirtualizationService {
    #[cfg(unix)]
    connection: spawn_unix::UnixConnection,
    #[cfg(windows)]
    rpc_port: u32,
    #[cfg(windows)]
    process: OwnedHandle,
    #[cfg(windows)]
    terminate_on_drop: bool,
}

/// Error handling function for `get_virtualization_service`.
///
/// # Safety
/// `message` shouldn't be used outside of the lifetime of the function. Management of `ctx` is
/// entirely up to the function.
pub type ErrorCallback =
    unsafe extern "C" fn(code: c_int, message: *const c_char, ctx: *mut c_void);

/// Spawns a new instance of virtmgr and rerturns a file descriptor for the socket connection to
/// the service. When error occurs, it is reported via the ErrorCallback function along with the
/// error message and any context that is set by the client.
///
/// # Safety
/// `cb` should be null or a valid function pointer of type `ErrorCallback`
#[no_mangle]
pub unsafe extern "C" fn get_virtualization_service(
    cb: Option<ErrorCallback>,
    ctx: *mut c_void,
) -> c_int {
    match VirtualizationService::new() {
        Ok(vs) => {
            #[cfg(unix)]
            {
                match vs.connection {
                    spawn_unix::UnixConnection::Bootstrap(client_fd) => client_fd.into_raw_fd(),
                    spawn_unix::UnixConnection::UnixDomain(path) => {
                        UnixStream::connect(path).map(|stream| stream.into_raw_fd()).unwrap_or(-1)
                    }
                }
            }
            #[cfg(windows)]
            {
                let _ = vs;
                if let Some(cb) = cb {
                    let msg = CString::new(
                        "get_virtualization_service is not supported on Windows host builds",
                    )
                    .unwrap();
                    unsafe { cb(libc::ENOTSUP, msg.as_ptr(), ctx) };
                }
                -1
            }
        }
        Err(e) => {
            if let Some(cb) = cb {
                let code = e.raw_os_error().unwrap_or(-1);
                let msg = CString::new(e.to_string()).unwrap();
                // SAFETY: `cb` doesn't use `msg` outside of the lifetime of the function.
                // msg's lifetime is longer than `cb` as it is bound to a local variable.
                unsafe { cb(code, msg.as_ptr(), ctx) };
            }
            -1
        }
    }
}

impl VirtualizationService {
    /// Spawns a new instance of virtmgr, a child process that will host
    /// the VirtualizationService AIDL service.
    pub fn new() -> Result<VirtualizationService, io::Error> {
        #[cfg(unix)]
        {
            Self::new_with_path(std::ffi::OsStr::new(VIRTMGR_PATH))
        }
        #[cfg(windows)]
        {
            Self::new_with_path(std::ffi::OsStr::new(VIRTMGR_PATH_WIN))
        }
    }

    /// Spawns a new instance of early_virtmgr, a child process that will host
    /// the VirtualizationService AIDL service for early VMs.
    pub fn new_early() -> Result<VirtualizationService, io::Error> {
        #[cfg(unix)]
        {
            Self::new_with_path(std::ffi::OsStr::new(EARLY_VIRTMGR_PATH))
        }
        #[cfg(windows)]
        {
            Self::new_with_path(std::ffi::OsStr::new(EARLY_VIRTMGR_PATH_WIN))
        }
    }

    fn new_with_path(virtmgr_path: &std::ffi::OsStr) -> Result<VirtualizationService, io::Error> {
        #[cfg(unix)]
        {
            let spawned = spawn_unix::spawn_virtmgr(virtmgr_path)?;
            Ok(VirtualizationService { connection: spawned.connection })
        }
        #[cfg(windows)]
        {
            let spawned = spawn_windows::spawn_virtmgr(virtmgr_path)?;
            Ok(VirtualizationService {
                rpc_port: spawned.rpc_port,
                process: spawned.process,
                terminate_on_drop: spawned.terminate_on_drop,
            })
        }
    }

    /// Connects to the VirtualizationService AIDL service.
    pub fn connect(&self) -> Result<Strong<dyn IVirtualizationService>, io::Error> {
        let session = RpcSession::new();
        session.set_file_descriptor_transport_mode(FileDescriptorTransportMode::Unix);
        session.set_max_incoming_threads(VIRTMGR_THREADS);
        #[cfg(unix)]
        {
            match &self.connection {
                spawn_unix::UnixConnection::Bootstrap(client_fd) => session
                    .setup_unix_domain_bootstrap_client(client_fd.as_fd())
                    .map_err(|_| io::Error::from(io::ErrorKind::ConnectionRefused)),
                spawn_unix::UnixConnection::UnixDomain(path) => session
                    .setup_unix_domain_client(path.to_string_lossy().as_ref())
                    .map_err(|_| io::Error::from(io::ErrorKind::ConnectionRefused)),
            }
        }
        #[cfg(windows)]
        {
            let deadline = std::time::Instant::now() + Duration::from_secs(5);
            loop {
                eprintln!("vmclient: before setup_vsock_client");
                match session.setup_vsock_client(HOST_RPC_CID, self.rpc_port) {
                    Ok(service) => {
                        eprintln!("vmclient: after setup_vsock_client");
                        break Ok(service);
                    }
                    Err(_) if std::time::Instant::now() < deadline => {
                        std::thread::sleep(Duration::from_millis(100));
                    }
                    Err(_) => break Err(io::Error::from(io::ErrorKind::ConnectionRefused)),
                }
            }
        }
    }
}

#[cfg(windows)]
impl Drop for VirtualizationService {
    fn drop(&mut self) {
        if !self.terminate_on_drop {
            return;
        }
        let mut exit_code = 0;
        let ok = unsafe { GetExitCodeProcess(self.process.as_raw_handle().cast(), &mut exit_code) };
        if ok != 0 && exit_code == STILL_ACTIVE_EXIT_CODE {
            let _ = unsafe { TerminateProcess(self.process.as_raw_handle().cast(), 0) };
        }
    }
}

/// A virtual machine which has been started by the VirtualizationService.
pub struct VmInstance {
    /// The `IVirtualMachine` Binder object representing the VM.
    pub vm: Strong<dyn IVirtualMachine>,
    cid: i32,
    state: Arc<Monitor<VmState>>,
    // Ensure that the DeathRecipient isn't dropped while someone might call wait_for_death, as it
    // is removed from the Binder when it's dropped.
    _death_recipient: DeathRecipient,
}

/// A trait to be implemented by clients to handle notification of significant changes to the VM
/// state. Default implementations of all functions are provided so clients only need to handle the
/// notifications they are interested in.
#[allow(unused_variables)]
pub trait VmCallback {
    /// Called when the payload has been started within the VM. If present, `stream` is connected
    /// to the stdin/stdout of the payload.
    fn on_payload_started(&self, cid: i32) {}

    /// Callend when the payload has notified Virtualization Service that it is ready to serve
    /// clients.
    fn on_payload_ready(&self, cid: i32) {}

    /// Called when the payload has exited in the VM. `exit_code` is the exit code of the payload
    /// process.
    fn on_payload_finished(&self, cid: i32, exit_code: i32) {}

    /// Called when an error has occurred in the VM. The `error_code` and `message` may give
    /// further details.
    fn on_error(&self, cid: i32, error_code: ErrorCode, message: &str) {}

    /// Called when the VM has exited, all resources have been freed, and any logs have been
    /// written. `death_reason` gives an indication why the VM exited.
    fn on_died(&self, cid: i32, death_reason: DeathReason) {}
}

impl VmInstance {
    /// Creates (but doesn't start) a new VM with the given configuration.
    pub fn create(
        service: &dyn IVirtualizationService,
        config: &VirtualMachineConfig,
        console_out: Option<File>,
        console_in: Option<File>,
        log: Option<File>,
        callback: Option<Box<dyn VmCallback + Send + Sync>>,
    ) -> BinderResult<Self> {
        let console_out = console_out.map(ParcelFileDescriptor::new);
        let console_in = console_in.map(ParcelFileDescriptor::new);
        let log = log.map(ParcelFileDescriptor::new);

        println!("vmclient: before service.createVm");
        let vm =
            service.createVm(config, console_out.as_ref(), console_in.as_ref(), log.as_ref())?;
        println!("vmclient: after service.createVm");

        println!("vmclient: before vm.getCid");
        let cid = vm.getCid()?;
        println!("vmclient: after vm.getCid cid={cid}");

        // Register callback before starting VM, in case it dies immediately.
        let state = Arc::new(Monitor::new(VmState::default()));
        let callback = BnVirtualMachineCallback::new_binder(
            VirtualMachineCallback { state: state.clone(), client_callback: callback },
            BinderFeatures::default(),
        );
        println!("vmclient: before vm.registerCallback");
        vm.registerCallback(&callback)?;
        println!("vmclient: after vm.registerCallback");
        println!("vmclient: before wait_for_binder_death");
        let death_recipient = wait_for_binder_death(&mut vm.as_binder(), state.clone())?;
        println!("vmclient: after wait_for_binder_death");

        Ok(Self { vm, cid, state, _death_recipient: death_recipient })
    }

    /// Starts the VM.
    pub fn start(&self) -> BinderResult<()> {
        self.vm.start()
    }

    /// Returns the VM context identifier used with guest RPC transports.
    ///
    /// On **Linux / Android** hosts this is the vsock CID. On **Windows**, virtmgr maps `(cid, port)`
    /// to named pipes `\\.\pipe\binder_rpc_vsock_{cid}_{port}` (see virtmgr `vsock_transport`).
    pub fn cid(&self) -> i32 {
        self.cid
    }

    /// Returns the current lifecycle state of the VM.
    pub fn state(&self) -> BinderResult<VirtualMachineState> {
        self.vm.getState()
    }

    /// Blocks until the VM or the VirtualizationService itself dies, and then returns the reason
    /// why it died.
    pub fn wait_for_death(&self) -> DeathReason {
        debug_trace(format!("vmclient: wait_for_death enter cid={}", self.cid));
        let reason = self
            .state
            .wait_while(|state| state.death_reason.is_none())
            .unwrap()
            .death_reason
            .unwrap();
        debug_trace(format!("vmclient: wait_for_death exit cid={} reason={:?}", self.cid, reason));
        reason
    }

    /// Blocks until the VM or the VirtualizationService itself dies, or the given timeout expires.
    /// Returns the reason why it died if it did so.
    pub fn wait_for_death_with_timeout(&self, timeout: Duration) -> Option<DeathReason> {
        let (state, _timeout_result) =
            self.state.wait_timeout_while(timeout, |state| state.death_reason.is_none()).unwrap();
        // We don't care if it timed out - we just return the reason if there now is one
        state.death_reason
    }

    /// Waits until the VM reports that it is ready.
    ///
    /// Returns an error if the VM dies first, or the `timeout` elapses before the VM is ready.
    pub fn wait_until_ready(&self, timeout: Duration) -> Result<(), VmWaitError> {
        let (state, timeout_result) = self
            .state
            .wait_timeout_while(timeout, |state| {
                state.reported_state < VirtualMachineState::READY && state.death_reason.is_none()
            })
            .unwrap();
        if timeout_result.timed_out() {
            Err(VmWaitError::TimedOut)
        } else if let Some(reason) = state.death_reason {
            Err(VmWaitError::Died { reason })
        } else if state.reported_state != VirtualMachineState::READY {
            Err(VmWaitError::Finished)
        } else {
            Ok(())
        }
    }

    /// Connects to an RPC Binder service in the guest listening on `port` (guest vsock port on
    /// Linux/Android; same port number participates in the Windows named-pipe mapping).
    ///
    /// Uses the Binder API `IVirtualMachine::connectVsock` on the host. **macOS** uses UDS paths
    /// emulating vsock `(cid, port)`; the VM stack must connect guest vsock to those paths (e.g. via
    /// the VMM) for end-to-end guest RPC.
    pub fn connect_service<T: FromIBinder + ?Sized>(
        &self,
        port: u32,
    ) -> Result<Strong<T>, StatusCode> {
        RpcSession::new().setup_preconnected_client(|| match self.vm.connectVsock(port as i32) {
            Ok(pfd) => pfd_from_connect_vsock_for_rpc(pfd),
            Err(e) => {
                warn!("connectVsock failed: {}", e);
                None
            }
        })
    }

    #[cfg(not(target_os = "android"))]
    pub fn start_tcp_vsock_bridge(&self, listen_port: u16, guest_port: u32) -> io::Result<()> {
        self.vm
            .startHostVsockTcpBridge(listen_port as i32, guest_port as i32)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
    }

    #[cfg(not(target_os = "android"))]
    pub fn set_host_console_name(&self, host_console_name: &str) -> io::Result<()> {
        self.vm
            .setHostConsoleName(host_console_name)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
    }
}

impl Debug for VmInstance {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("VmInstance").field("cid", &self.cid).field("state", &self.state).finish()
    }
}

/// Notify the VmState when the given Binder object dies.
///
/// If the returned DeathRecipient is dropped then this will no longer do anything.
fn wait_for_binder_death(
    binder: &mut impl IBinder,
    state: Arc<Monitor<VmState>>,
) -> BinderResult<DeathRecipient> {
    let mut death_recipient = DeathRecipient::new(move || {
        warn!("VirtualizationService unexpectedly died");
        state.notify_death(DeathReason::VirtualizationServiceDied);
    });
    binder.link_to_death(&mut death_recipient)?;
    Ok(death_recipient)
}

#[derive(Debug, Default)]
struct VmState {
    death_reason: Option<DeathReason>,
    reported_state: VirtualMachineState,
}

impl Monitor<VmState> {
    fn notify_death(&self, reason: DeathReason) {
        let state = &mut *self.state.lock().unwrap();
        // In case this method is called more than once, ignore subsequent calls.
        if state.death_reason.is_none() {
            state.death_reason.replace(reason);
            self.cv.notify_all();
        }
    }

    fn notify_state(&self, state: VirtualMachineState) {
        self.state.lock().unwrap().reported_state = state;
        self.cv.notify_all();
    }
}

struct VirtualMachineCallback {
    state: Arc<Monitor<VmState>>,
    client_callback: Option<Box<dyn VmCallback + Send + Sync>>,
}

impl Debug for VirtualMachineCallback {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("VirtualMachineCallback")
            .field("state", &self.state)
            .field(
                "client_callback",
                &if self.client_callback.is_some() { "Some(...)" } else { "None" },
            )
            .finish()
    }
}

impl Interface for VirtualMachineCallback {}

impl IVirtualMachineCallback for VirtualMachineCallback {
    fn onPayloadStarted(&self, cid: i32) -> BinderResult<()> {
        self.state.notify_state(VirtualMachineState::STARTED);
        if let Some(ref callback) = self.client_callback {
            callback.on_payload_started(cid);
        }
        Ok(())
    }

    fn onPayloadReady(&self, cid: i32) -> BinderResult<()> {
        self.state.notify_state(VirtualMachineState::READY);
        if let Some(ref callback) = self.client_callback {
            callback.on_payload_ready(cid);
        }
        Ok(())
    }

    fn onPayloadFinished(&self, cid: i32, exit_code: i32) -> BinderResult<()> {
        self.state.notify_state(VirtualMachineState::FINISHED);
        if let Some(ref callback) = self.client_callback {
            callback.on_payload_finished(cid, exit_code);
        }
        Ok(())
    }

    fn onError(&self, cid: i32, error_code: AidlErrorCode, message: &str) -> BinderResult<()> {
        self.state.notify_state(VirtualMachineState::FINISHED);
        if let Some(ref callback) = self.client_callback {
            let error_code = error_code.into();
            callback.on_error(cid, error_code, message);
        }
        Ok(())
    }

    fn onDied(&self, cid: i32, reason: AidlDeathReason) -> BinderResult<()> {
        let reason = reason.into();
        debug_trace(format!("vmclient: onDied cid={} reason={:?}", cid, reason));
        self.state.notify_death(reason);
        if let Some(ref callback) = self.client_callback {
            callback.on_died(cid, reason);
        }
        Ok(())
    }
}
