// Copyright 2023, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.

//! Access to hypervisor capabilities via system properties set by the bootloader.
//!
//! The full Android build links `libplatformproperties_rust` and reads device properties.
//! This workspace snapshot provides stubs so `cargo check` for host tools succeeds.

use anyhow::Result;

/// Returns whether there is a hypervisor present that supports non-protected VMs.
pub fn is_vm_supported() -> Result<bool> {
    Ok(true)
}

/// Returns whether there is a hypervisor present that supports protected VMs.
pub fn is_protected_vm_supported() -> Result<bool> {
    Ok(!cfg!(all(target_os = "windows", target_arch = "x86_64")))
}

/// Returns whether there is a hypervisor present that supports any sort of VM, either protected
/// or non-protected.
pub fn is_any_vm_supported() -> Result<bool> {
    is_vm_supported().and_then(|ok| if ok { Ok(true) } else { is_protected_vm_supported() })
}

/// Returns the version of the hypervisor, if there is one.
pub fn version() -> Result<Option<String>> {
    Ok(None)
}
