//! virtmgr构建脚本
//!
//! 处理平台特定的构建配置。

use std::env;
use std::path::PathBuf;

fn main() {
    // 设置平台特定的配置
    let target = env::var("TARGET").unwrap_or_default();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // 根据目标平台设置不同的配置
    if target.contains("windows") {
        println!("cargo:rustc-cfg=feature=\"windows\"");

        // Windows特定配置
    } else if target.contains("linux") || target.contains("android") {
        println!("cargo:rustc-cfg=feature=\"unix\"");

        if target.contains("android") {
            println!("cargo:rustc-cfg=feature=\"android\"");
        }
    }

    // 添加项目特定的cfg配置（与 AOSP Soong `rustflags` 中的 `--cfg foo` 对齐；主机 cargo 构建需声明以免 `unexpected_cfgs`）
    println!("cargo:rustc-check-cfg=cfg(early)");
    println!("cargo:rustc-check-cfg=cfg(tpu_assignable_device)");
    println!("cargo:rustc-check-cfg=cfg(paravirtualized_devices)");
    println!("cargo:rustc-check-cfg=cfg(network)");
    println!("cargo:rustc-check-cfg=cfg(llpvm_changes)");
    println!("cargo:rustc-check-cfg=cfg(vendor_modules)");
    println!("cargo:rustc-check-cfg=cfg(device_assignment)");
    println!("cargo:rustc-check-cfg=cfg(multi_tenant)");
    println!("cargo:rustc-check-cfg=cfg(debuggable_vms_improvements)");
    println!("cargo:rustc-check-cfg=cfg(virt_cpufreq)");

    // 根据环境变量设置early配置
    if env::var("VIRTMGR_EARLY").unwrap_or_default() == "1" {
        println!("cargo:rustc-cfg=early");
    }

    // 根据环境变量设置tpu_assignable_device配置
    if env::var("VIRTMGR_TPU_ASSIGNABLE_DEVICE").unwrap_or_default() == "1" {
        println!("cargo:rustc-cfg=tpu_assignable_device");
    }

    // Ensure host Cargo builds can always find binder-rpc copied by root build script.
    // Priority:
    // 1) VIRTMGR_BINDER_RPC_LIB_DIR (explicit override)
    // 2) <repo>/frameworks/native/libs/binder/rust/sys/libs (default copy destination)
    let binder_rpc_lib_dir = env::var("VIRTMGR_BINDER_RPC_LIB_DIR").unwrap_or_else(|_| {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        manifest_dir
            .join("../../../../..")
            .join("frameworks/native/libs/binder/rust/sys/libs")
            .to_string_lossy()
            .into_owned()
    });
    println!("cargo:rustc-link-search=native={binder_rpc_lib_dir}");
    println!("cargo:rustc-link-lib=dylib=binder-rpc");

    // 设置输出目录
    let _out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
}
