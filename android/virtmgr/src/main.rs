// Copyright 2022, The Android Open Source Project
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

//! Android Virtualization Manager

mod aidl;
mod atom;
mod composite;
mod crosvm;
mod debug_config;
mod dt_overlay;
#[cfg(not(target_os = "android"))]
mod host_internal_service;
mod non_windows_main;
mod os_compat;
mod payload;
mod selinux;
mod vsock_transport;

use crate::os_compat::pid_t;

#[cfg(unix)]
use nix::unistd::{Pid, Uid};
#[cfg(unix)]
use std::sync::LazyLock;

#[cfg(windows)]
fn getpid() -> pid_t {
    unsafe { libc::getpid() }
}

#[cfg(unix)]
static PID_CURRENT: LazyLock<Pid> = LazyLock::new(Pid::this);
#[cfg(unix)]
static PID_PARENT: LazyLock<Pid> = LazyLock::new(Pid::parent);
#[cfg(unix)]
static UID_CURRENT: LazyLock<Uid> = LazyLock::new(Uid::current);

pub fn get_this_pid() -> pid_t {
    #[cfg(unix)]
    {
        PID_CURRENT.as_raw()
    }
    #[cfg(windows)]
    {
        getpid()
    }
}

pub fn get_calling_pid() -> pid_t {
    #[cfg(unix)]
    {
        PID_PARENT.as_raw()
    }
    #[cfg(windows)]
    {
        getpid()
    }
}

#[cfg(unix)]
pub fn get_calling_uid() -> libc::uid_t {
    UID_CURRENT.as_raw()
}

#[cfg(windows)]
pub fn get_calling_uid() -> u32 {
    0
}

fn main() {
    non_windows_main::run();
}
