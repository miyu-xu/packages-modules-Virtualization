// Copyright 2022, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.

use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, Read};
use std::os::fd::{AsFd, RawFd};
use std::os::unix::io::{AsRawFd, OwnedFd};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use crate::debug_trace;
use command_fds::CommandFdExt;
use nix::errno::Errno;
#[cfg(not(target_os = "macos"))]
use nix::fcntl::OFlag;
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use nix::sys::signal;
use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
#[cfg(not(target_os = "macos"))]
use nix::unistd::pipe2;
use nix::unistd::{pipe, Pid};
use shared_child::SharedChild;

const VIRTMGR_SERVICE_DIR_ENV: &str = "VIRTMGR_SERVICE_DIR";
const VIRTMGR_PATH_ENV: &str = "VIRTMGR_PATH";
const VIRTMGR_SERVICE_STATE_FILE: &str = "virtmgr-service.state";
const VIRTMGR_SERVICE_SOCKET_FILE: &str = "virtmgr-service.sock";

pub enum UnixConnection {
    Bootstrap(OwnedFd),
    UnixDomain(PathBuf),
}

pub struct TransientVirtmgr {
    child: Option<SharedChild>,
}

impl TransientVirtmgr {
    fn new(child: SharedChild) -> Self {
        Self { child: Some(child) }
    }

    /// Returns whether the transient service process is still running.
    pub fn is_alive(&self) -> Option<bool> {
        self.child.as_ref().and_then(|child| child.try_wait().ok().map(|status| status.is_none()))
    }

    /// Transfers process lifetime to the returned bootstrap connection.
    ///
    /// The C ABI exposes only the connection fd and cannot retain this guard. Preserve its
    /// historical behavior there; Rust callers keep the guard and terminate their transient
    /// service deterministically.
    pub fn detach(mut self) {
        self.child.take();
    }
}

impl Drop for TransientVirtmgr {
    fn drop(&mut self) {
        let Some(child) = self.child.take() else {
            return;
        };
        if matches!(child.try_wait(), Ok(Some(_))) {
            return;
        }
        let pid = Pid::from_raw(child.id() as i32);
        let _ = signal::kill(pid, signal::Signal::SIGTERM);
        if !matches!(child.wait_timeout(Duration::from_secs(2)), Ok(Some(_))) {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

pub struct SpawnedVirtmgr {
    pub connection: UnixConnection,
    pub transient_process: Option<TransientVirtmgr>,
}

fn set_cloexec<F: AsFd>(fd: &F) -> io::Result<()> {
    fcntl(fd.as_fd().as_raw_fd(), FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC))
        .map_err(io::Error::from)?;
    Ok(())
}

pub fn posix_pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    #[cfg(target_os = "macos")]
    {
        let (read_fd, write_fd) = pipe()?;
        set_cloexec(&read_fd)?;
        set_cloexec(&write_fd)?;
        Ok((read_fd, write_fd))
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(pipe2(OFlag::O_CLOEXEC)?)
    }
}

pub fn posix_socketpair() -> io::Result<(OwnedFd, OwnedFd)> {
    #[cfg(target_os = "macos")]
    {
        let (client_fd, server_fd) =
            socketpair(AddressFamily::Unix, SockType::Stream, None, SockFlag::empty())?;
        set_cloexec(&client_fd)?;
        set_cloexec(&server_fd)?;
        Ok((client_fd, server_fd))
    }
    #[cfg(not(target_os = "macos"))]
    {
        Ok(socketpair(AddressFamily::Unix, SockType::Stream, None, SockFlag::SOCK_CLOEXEC)?)
    }
}

fn service_dir() -> Option<PathBuf> {
    std::env::var_os(VIRTMGR_SERVICE_DIR_ENV).filter(|path| !path.is_empty()).map(PathBuf::from)
}

fn service_state_path(service_dir: &Path) -> PathBuf {
    service_dir.join(VIRTMGR_SERVICE_STATE_FILE)
}

fn service_socket_path(service_dir: &Path) -> PathBuf {
    service_dir.join(VIRTMGR_SERVICE_SOCKET_FILE)
}

fn parse_service_state(contents: &str) -> Option<(i32, PathBuf)> {
    let mut pid = None;
    let mut socket_path = None;
    for line in contents.lines() {
        let (key, value) = line.split_once('=')?;
        match key.trim() {
            "pid" => pid = value.trim().parse::<i32>().ok(),
            "socket_path" => socket_path = Some(PathBuf::from(value.trim())),
            _ => {}
        }
    }
    Some((pid?, socket_path?))
}

fn write_service_state(service_dir: &Path, pid: i32, socket_path: &Path) -> io::Result<()> {
    fs::create_dir_all(service_dir)?;
    let contents = format!("pid={pid}\nsocket_path={}\n", socket_path.display());
    fs::write(service_state_path(service_dir), contents)
}

fn remove_service_artifacts(service_dir: &Path) {
    let _ = fs::remove_file(service_state_path(service_dir));
    let _ = fs::remove_file(service_socket_path(service_dir));
}

fn process_alive(pid: i32) -> bool {
    match signal::kill(Pid::from_raw(pid), None) {
        Ok(()) => true,
        Err(Errno::EPERM) => true,
        Err(_) => false,
    }
}

fn try_open_existing_service(service_dir: &Path) -> io::Result<Option<SpawnedVirtmgr>> {
    let contents = match fs::read_to_string(service_state_path(service_dir)) {
        Ok(contents) => contents,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err),
    };
    let Some((pid, socket_path)) = parse_service_state(&contents) else {
        remove_service_artifacts(service_dir);
        return Ok(None);
    };
    if process_alive(pid) && socket_path.exists() {
        return Ok(Some(SpawnedVirtmgr {
            connection: UnixConnection::UnixDomain(socket_path),
            transient_process: None,
        }));
    }
    remove_service_artifacts(service_dir);
    Ok(None)
}

fn spawn_transient_virtmgr(virtmgr_path: &OsStr) -> io::Result<SpawnedVirtmgr> {
    let (wait_fd, ready_fd) = posix_pipe()?;
    let (client_fd, server_fd) = posix_socketpair()?;
    debug_trace(format!(
        "vmclient: spawning transient virtmgr path={} server_fd={} ready_fd={}",
        virtmgr_path.to_string_lossy(),
        server_fd.as_raw_fd(),
        ready_fd.as_raw_fd()
    ));

    let mut command = Command::new(virtmgr_path);
    command.arg("--rpc-server-fd").arg(format!("{}", server_fd.as_raw_fd()));
    command.arg("--ready-fd").arg(format!("{}", ready_fd.as_raw_fd()));
    command.preserved_fds(vec![server_fd, ready_fd]);

    let child = SharedChild::spawn(&mut command)?;
    drop(command);
    debug_trace(format!("vmclient: transient virtmgr spawned pid={}", child.id()));

    debug_trace("vmclient: waiting for transient virtmgr ready byte");
    let _ = File::from(wait_fd).read(&mut [0])?;
    debug_trace("vmclient: transient virtmgr ready byte received");
    Ok(SpawnedVirtmgr {
        connection: UnixConnection::Bootstrap(client_fd),
        transient_process: Some(TransientVirtmgr::new(child)),
    })
}

fn spawn_persistent_virtmgr(
    virtmgr_path: &OsStr,
    service_dir: &Path,
) -> io::Result<SpawnedVirtmgr> {
    fs::create_dir_all(service_dir)?;
    let socket_path = service_socket_path(service_dir);
    let _ = fs::remove_file(&socket_path);

    let (wait_fd, ready_fd) = posix_pipe()?;
    debug_trace(format!(
        "vmclient: spawning persistent virtmgr path={} socket_path={} ready_fd={}",
        virtmgr_path.to_string_lossy(),
        socket_path.display(),
        ready_fd.as_raw_fd()
    ));
    let mut command = Command::new(virtmgr_path);
    command.arg("--rpc-server-path").arg(&socket_path);
    command.arg("--ready-fd").arg(format!("{}", ready_fd.as_raw_fd()));
    command.preserved_fds(vec![ready_fd]);

    let child = SharedChild::spawn(&mut command)?;
    drop(command);
    debug_trace(format!("vmclient: persistent virtmgr spawned pid={}", child.id()));
    debug_trace("vmclient: waiting for persistent virtmgr ready byte");
    let _ = File::from(wait_fd).read(&mut [0])?;
    debug_trace("vmclient: persistent virtmgr ready byte received");
    write_service_state(service_dir, child.id() as i32, &socket_path)?;
    debug_trace(format!(
        "vmclient: wrote service state pid={} socket_path={}",
        child.id(),
        socket_path.display()
    ));

    Ok(SpawnedVirtmgr {
        connection: UnixConnection::UnixDomain(socket_path),
        transient_process: None,
    })
}

pub fn spawn_virtmgr(virtmgr_path: &OsStr) -> io::Result<SpawnedVirtmgr> {
    let exe_owned: OsString = std::env::var_os(VIRTMGR_PATH_ENV)
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| virtmgr_path.to_owned());
    let exe = exe_owned.as_os_str();
    debug_trace(format!("vmclient: resolved virtmgr path={}", exe.to_string_lossy()));

    if let Some(service_dir) = service_dir() {
        debug_trace(format!("vmclient: persistent service mode dir={}", service_dir.display()));
        if let Some(existing) = try_open_existing_service(&service_dir)? {
            debug_trace("vmclient: reusing existing virtmgr service");
            return Ok(existing);
        }
        return spawn_persistent_virtmgr(exe, &service_dir);
    }

    debug_trace("vmclient: transient service mode");
    spawn_transient_virtmgr(exe)
}

pub fn connect_fd_to_raw(fd: &OwnedFd) -> RawFd {
    fd.as_raw_fd()
}
