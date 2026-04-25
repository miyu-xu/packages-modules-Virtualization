// Entry point implementation (runs on all targets including Windows MinGW).

use crate::aidl::{GLOBAL_SERVICE, VirtualizationService};
use android_system_virtualizationservice::aidl::android::system::virtualizationservice::IVirtualizationService::BnVirtualizationService;
use anyhow::{bail, Result};
use binder::{BinderFeatures, ProcessState};
use clap::Parser;
use log::{info, LevelFilter};
use rpcbinder::{FileDescriptorTransportMode, RpcServer};

#[cfg(unix)]
use nix::unistd::write;
#[cfg(unix)]
use rustutils::inherited_fd::take_fd_ownership;
#[cfg(unix)]
use std::os::fd::{BorrowedFd, FromRawFd, IntoRawFd, OwnedFd};
#[cfg(unix)]
use std::os::unix::io::RawFd;
#[cfg(unix)]
use std::os::unix::net::UnixListener;
#[cfg(unix)]
use std::path::PathBuf;
#[cfg(windows)]
use windows_sys::Win32::Networking::WinSock::{WSAStartup, WSADATA};
#[cfg(windows)]
const HOST_RPC_CID: u32 = 1;

const LOG_TAG: &str = "virtmgr";

#[cfg(windows)]
fn ensure_winsock_init() {
    let mut data: WSADATA = unsafe { std::mem::zeroed() };
    let rc = unsafe { WSAStartup(0x0202, &mut data) };
    if rc != 0 {
        panic!("WSAStartup failed: {rc}");
    }
}

#[derive(Parser)]
struct Args {
    #[cfg(unix)]
    #[clap(long)]
    rpc_server_fd: Option<RawFd>,
    #[cfg(unix)]
    #[clap(long)]
    rpc_server_path: Option<PathBuf>,
    #[cfg(unix)]
    #[clap(long)]
    ready_fd: RawFd,
    #[cfg(windows)]
    #[clap(long)]
    rpc_port: u16,
}

fn check_vm_support() -> Result<()> {
    if hypervisor_props::is_any_vm_supported()? {
        Ok(())
    } else {
        bail!("Device doesn't support protected or non-protected VMs")
    }
}

pub fn run() {
    unsafe { rustutils::inherited_fd::init_once() }
        .expect("Failed to take ownership of inherited FDs");

    android_logger::init_once(
        android_logger::Config::default()
            .with_tag(LOG_TAG)
            .with_max_level(LevelFilter::Info)
            .with_log_buffer(android_logger::LogId::System),
    );

    check_vm_support().unwrap();

    #[cfg(windows)]
    ensure_winsock_init();

    let args = Args::parse();
    #[cfg(unix)]
    let ready_fd = take_fd_ownership(args.ready_fd).expect("Failed to take ownership of ready_fd");
    #[cfg(unix)]
    let rpc_server = match (args.rpc_server_fd, args.rpc_server_path) {
        (Some(rpc_server_fd), None) => Some((false, unsafe {
            OwnedFd::from_raw_fd(
                take_fd_ownership(rpc_server_fd)
                    .expect("Failed to take ownership of rpc_server_fd"),
            )
        })),
        (None, Some(rpc_server_path)) => {
            let _ = std::fs::remove_file(&rpc_server_path);
            let listener = UnixListener::bind(&rpc_server_path).unwrap_or_else(|err| {
                panic!("Failed to bind rpc server path {}: {err}", rpc_server_path.display())
            });
            let listener_fd = unsafe { OwnedFd::from_raw_fd(listener.into_raw_fd()) };
            Some((true, listener_fd))
        }
        _ => panic!("Exactly one of --rpc-server-fd or --rpc-server-path must be provided"),
    };

    #[cfg(not(windows))]
    ProcessState::start_thread_pool();

    #[cfg(unix)]
    {
        if cfg!(early) {
            let pid = i32::from(crate::get_this_pid());
            let lim = libc::rlimit { rlim_cur: libc::RLIM_INFINITY, rlim_max: libc::RLIM_INFINITY };
            let ret =
                unsafe { libc::prlimit(pid, libc::RLIMIT_MEMLOCK, &lim, std::ptr::null_mut()) };
            if ret == -1 {
                panic!("rlimit error: {}", std::io::Error::last_os_error());
            } else if ret != 0 {
                panic!("Unexpected return value from prlimit(): {ret}");
            }
        } else {
            GLOBAL_SERVICE.removeMemlockRlimit().expect("Failed to remove memlock rlimit");
        }
    }
    #[cfg(windows)]
    {
        if !cfg!(early) {
            GLOBAL_SERVICE.removeMemlockRlimit().expect("Failed to remove memlock rlimit");
        }
    }

    let service = VirtualizationService::init();
    let service =
        BnVirtualizationService::new_binder(service, BinderFeatures::default()).as_binder();
    #[cfg(unix)]
    let server = match rpc_server.expect("unix rpc server config must be present") {
        (true, rpc_server_fd) => {
            RpcServer::new_bound_socket(service, rpc_server_fd).expect("Failed to start RpcServer")
        }
        (false, rpc_server_fd) => RpcServer::new_unix_domain_bootstrap(service, rpc_server_fd)
            .expect("Failed to start RpcServer"),
    };
    #[cfg(windows)]
    let server = RpcServer::new_vsock(service, HOST_RPC_CID, args.rpc_port as u32)
        .expect("Failed to start RpcServer");
    server.set_supported_file_descriptor_transport_modes(&[FileDescriptorTransportMode::Unix]);

    info!("Started VirtualizationService RpcServer. Ready to accept connections");

    #[cfg(unix)]
    {
        let ready_fd = unsafe { BorrowedFd::borrow_raw(ready_fd) };
        write(ready_fd, "o".as_bytes())
            .expect("Failed to write a single character through ready_fd");
    }
    server.join();
    info!("Shutting down VirtualizationService RpcServer");
}
