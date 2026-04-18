// Copyright 2022, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.

use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Read};
use std::os::fd::RawFd;
use std::os::unix::io::{AsFd, AsRawFd, IntoRawFd, OwnedFd};
use std::process::Command;

use command_fds::CommandFdExt;
use nix::fcntl::OFlag;
use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
use nix::unistd::pipe2;
use shared_child::SharedChild;

pub fn posix_pipe() -> io::Result<(OwnedFd, OwnedFd)> {
    Ok(pipe2(OFlag::O_CLOEXEC)?)
}

pub fn posix_socketpair() -> io::Result<(OwnedFd, OwnedFd)> {
    Ok(socketpair(AddressFamily::Unix, SockType::Stream, None, SockFlag::SOCK_CLOEXEC)?)
}

pub fn spawn_virtmgr(virtmgr_path: &OsStr) -> io::Result<OwnedFd> {
    let (wait_fd, ready_fd) = posix_pipe()?;
    let (client_fd, server_fd) = posix_socketpair()?;

    let mut command = Command::new(virtmgr_path);
    command.arg("--rpc-server-fd").arg(format!("{}", server_fd.as_raw_fd()));
    command.arg("--ready-fd").arg(format!("{}", ready_fd.as_raw_fd()));
    command.preserved_fds(vec![server_fd, ready_fd]);

    SharedChild::spawn(&mut command)?;

    let _ = File::from(wait_fd).read(&mut [0])?;
    Ok(client_fd)
}

pub fn connect_fd_to_raw(fd: &OwnedFd) -> RawFd {
    fd.as_raw_fd()
}
