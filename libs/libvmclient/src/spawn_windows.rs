// Copyright 2022, The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.

//! Spawn `virtmgr` on Windows using a named-pipe-backed vsock RPC bootstrap.
use crate::debug_trace;

use std::ffi::{OsStr, OsString};
use std::fs;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::path::{Path, PathBuf};
use std::ptr;

use winapi::shared::minwindef::FALSE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{
    CreateProcessW, GetExitCodeProcess, OpenProcess, PROCESS_INFORMATION, STARTUPINFOW,
};
use winapi::um::winbase::CREATE_UNICODE_ENVIRONMENT;
use winapi::um::winnt::PROCESS_QUERY_LIMITED_INFORMATION;

const VIRTMGR_SERVICE_DIR_ENV: &str = "VIRTMGR_SERVICE_DIR";
const VIRTMGR_SERVICE_STATE_FILE: &str = "virtmgr-service.state";
const STILL_ACTIVE_EXIT_CODE: u32 = 259;

fn wide_nul(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(Some(0)).collect()
}

pub struct SpawnedVirtmgr {
    pub rpc_port: u32,
    pub process: OwnedHandle,
    pub terminate_on_drop: bool,
}

fn service_dir() -> Option<PathBuf> {
    let value = std::env::var_os(VIRTMGR_SERVICE_DIR_ENV);
    debug_trace(format!(
        "vmclient: VIRTMGR_SERVICE_DIR raw={:?}",
        value.as_ref().map(PathBuf::from)
    ));
    value.filter(|path| !path.is_empty()).map(PathBuf::from)
}

fn service_state_path(service_dir: &Path) -> PathBuf {
    service_dir.join(VIRTMGR_SERVICE_STATE_FILE)
}

fn parse_service_state(contents: &str) -> Option<(u32, u32)> {
    let mut pid = None;
    let mut rpc_port = None;
    for line in contents.lines() {
        let (key, value) = line.split_once('=')?;
        match key.trim() {
            "pid" => pid = value.trim().parse::<u32>().ok(),
            "rpc_port" => rpc_port = value.trim().parse::<u32>().ok(),
            _ => {}
        }
    }
    Some((pid?, rpc_port?))
}

fn write_service_state(service_dir: &Path, pid: u32, rpc_port: u32) -> io::Result<()> {
    fs::create_dir_all(service_dir)?;
    let state_path = service_state_path(service_dir);
    let contents = format!("pid={pid}\nrpc_port={rpc_port}\n");
    debug_trace(format!(
        "vmclient: writing service state path={} pid={} rpc_port={}",
        state_path.display(),
        pid,
        rpc_port
    ));
    fs::write(state_path, contents)
}

fn remove_service_state(service_dir: &Path) {
    let _ = fs::remove_file(service_state_path(service_dir));
}

fn try_open_existing_service(service_dir: &Path) -> io::Result<Option<SpawnedVirtmgr>> {
    let state_path = service_state_path(service_dir);
    debug_trace(format!("vmclient: checking existing service state path={}", state_path.display()));
    let contents = match fs::read_to_string(&state_path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err),
    };
    let Some((pid, rpc_port)) = parse_service_state(&contents) else {
        remove_service_state(service_dir);
        return Ok(None);
    };

    let process = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid) };
    if process.is_null() {
        debug_trace(format!(
            "vmclient: existing service pid={} could not be opened; removing state",
            pid
        ));
        remove_service_state(service_dir);
        return Ok(None);
    }

    let process = unsafe { OwnedHandle::from_raw_handle(process.cast()) };
    let mut exit_code = 0u32;
    let ok = unsafe { GetExitCodeProcess(process.as_raw_handle().cast(), &mut exit_code) };
    if ok == 0 || exit_code != STILL_ACTIVE_EXIT_CODE {
        debug_trace(format!(
            "vmclient: existing service pid={} not active (ok={} exit_code={}); removing state",
            pid, ok, exit_code
        ));
        remove_service_state(service_dir);
        return Ok(None);
    }

    debug_trace(format!("vmclient: reusing existing virtmgr pid={} rpc_port={}", pid, rpc_port));
    Ok(Some(SpawnedVirtmgr { rpc_port, process, terminate_on_drop: false }))
}

fn spawn_new_virtmgr(exe: &OsStr) -> io::Result<(u32, OwnedHandle, u32)> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port() as u32;
    drop(listener);
    debug_trace(format!(
        "vmclient: spawning virtmgr exe={} rpc_port={}",
        exe.to_string_lossy(),
        port
    ));

    let cmdline = format!("\"{}\" --rpc-port {}", exe.to_string_lossy(), port);
    let mut cmd_wide: Vec<u16> = cmdline.encode_utf16().chain(std::iter::once(0)).collect();

    let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
    si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let app_wide = wide_nul(exe);

    let ok = unsafe {
        CreateProcessW(
            app_wide.as_ptr(),
            cmd_wide.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            FALSE,
            CREATE_UNICODE_ENVIRONMENT,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut si,
            &mut pi,
        )
    };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }

    unsafe {
        CloseHandle(pi.hThread);
    }

    let process = unsafe { OwnedHandle::from_raw_handle(pi.hProcess.cast()) };
    Ok((port, process, pi.dwProcessId))
}

pub fn spawn_virtmgr(virtmgr_exe: &OsStr) -> io::Result<SpawnedVirtmgr> {
    let exe_owned: OsString =
        std::env::var_os("VIRTMGR_PATH").filter(|path| !path.is_empty()).unwrap_or_else(|| {
            if virtmgr_exe.len() > 0 {
                virtmgr_exe.to_owned()
            } else {
                OsString::from("virtmgr.exe")
            }
        });
    let exe = exe_owned.as_os_str();
    debug_trace(format!("vmclient: resolved virtmgr path={}", exe.to_string_lossy()));

    if let Some(service_dir) = service_dir() {
        debug_trace(format!("vmclient: persistent service mode dir={}", service_dir.display()));
        fs::create_dir_all(&service_dir)?;
        if let Some(existing) = try_open_existing_service(&service_dir)? {
            return Ok(existing);
        }

        let (rpc_port, process, pid) = spawn_new_virtmgr(exe)?;
        write_service_state(&service_dir, pid, rpc_port)?;
        debug_trace(format!(
            "vmclient: started persistent virtmgr pid={} rpc_port={}",
            pid, rpc_port
        ));
        return Ok(SpawnedVirtmgr { rpc_port, process, terminate_on_drop: false });
    }

    let (rpc_port, process, _pid) = spawn_new_virtmgr(exe)?;
    debug_trace(format!("vmclient: started transient virtmgr rpc_port={}", rpc_port));
    Ok(SpawnedVirtmgr { rpc_port, process, terminate_on_drop: true })
}
