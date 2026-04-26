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

//! Initializes and exposes the platform-specific [`DesktopHost`] instance.
//!
//! Usage: call [`init()`] once during startup, then access via [`host()`].

use desktop_host::traits::DesktopHost;
use once_cell::sync::OnceCell;
use std::sync::Arc;

/// Global DesktopHost instance. Initialized by [`init()`].
static HOST: OnceCell<Arc<dyn DesktopHost>> = OnceCell::new();

/// Initialize the global DesktopHost for the current platform.
///
/// Must be called once during virtmgr startup before any platform-specific
/// APIs are used. Panics if called more than once.
pub fn init() {
    let platform = desktop_host::create_desktop_host();
    if HOST.set(Arc::new(platform)).is_err() {
        panic!("desktop_host::init() called more than once");
    }
    log::info!("DesktopHost initialized: {}", host().platform_name());
}

/// Return a reference to the global `DesktopHost` trait object.
///
/// # Panics
/// Panics if [`init()`] has not been called yet.
pub fn host() -> &'static dyn DesktopHost {
    HOST.get()
        .expect("desktop_host::init() must be called before accessing host()")
        .as_ref()
}
