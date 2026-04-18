//! VM control IPC for host-side virtmgr.
//!
//! On Unix targets we use a stub that returns `ENOTSUP` for balloon stats (matching Android when
//! the balloon is unavailable). On Windows hosts the client can delegate to the `crosvm` CLI for
//! suspend/resume and balloon adjustment when `VIRTMGR_CROSVM_PATH` points at a working binary.

use std::path::Path;

#[derive(Debug)]
pub struct SysError(i32);

impl SysError {
    pub fn errno(&self) -> i32 {
        self.0
    }
}

#[derive(Debug)]
pub enum BalloonControlCommand {
    Stats {},
    Adjust { num_bytes: u64, wait_for_success: bool },
}

#[derive(Debug)]
pub enum VmRequest {
    BalloonCommand(BalloonControlCommand),
    SuspendVcpus,
    ResumeVcpus,
    ConnectVsock { port: u32 },
}

#[derive(Debug)]
pub enum VmResponse {
    BalloonStats {
        stats: (),
        balloon_actual: u64,
    },
    Err(SysError),
    Ok,
}

#[cfg(not(windows))]
pub mod client {
    use super::{BalloonControlCommand, SysError, VmRequest, VmResponse};
    use std::io::Error;
    use std::path::Path;

    pub fn handle_request(req: &VmRequest, _path: &Path) -> Result<VmResponse, Error> {
        match req {
            VmRequest::BalloonCommand(BalloonControlCommand::Stats {}) => Ok(VmResponse::Err(
                SysError(libc::ENOTSUP),
            )),
            _ => Ok(VmResponse::Ok),
        }
    }
}

#[cfg(windows)]
pub mod client {
    use super::{BalloonControlCommand, SysError, VmRequest, VmResponse};
    use serde_json::Value;
    use std::io::Error;
    use std::path::Path;
    use std::process::Command;

    fn crosvm_exe() -> std::ffi::OsString {
        std::env::var_os("VIRTMGR_CROSVM_PATH").unwrap_or_else(|| "crosvm.exe".into())
    }

    fn find_balloon_actual(v: &Value) -> Option<u64> {
        match v {
            Value::Object(map) => {
                if let Some(n) = map.get("balloon_actual").and_then(|x| x.as_u64()) {
                    return Some(n);
                }
                for child in map.values() {
                    if let Some(n) = find_balloon_actual(child) {
                        return Some(n);
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Uses the `crosvm` CLI for operations that map to subcommands; `balloon_stats` parses JSON
    /// from stdout (same shape as `crosvm balloon_stats` / `VmResponse`).
    pub fn handle_request(req: &VmRequest, path: &Path) -> Result<VmResponse, Error> {
        let socket = path.to_string_lossy();
        match req {
            VmRequest::SuspendVcpus => {
                let status = Command::new(crosvm_exe())
                    .arg("suspend")
                    .arg(socket.as_ref())
                    .status()?;
                if status.success() {
                    Ok(VmResponse::Ok)
                } else {
                    Err(Error::other(format!(
                        "crosvm suspend failed with status {}",
                        status
                    )))
                }
            }
            VmRequest::ResumeVcpus => {
                let status = Command::new(crosvm_exe())
                    .arg("resume")
                    .arg(socket.as_ref())
                    .status()?;
                if status.success() {
                    Ok(VmResponse::Ok)
                } else {
                    Err(Error::other(format!(
                        "crosvm resume failed with status {}",
                        status
                    )))
                }
            }
            VmRequest::ConnectVsock { port } => {
                let status = Command::new(crosvm_exe())
                    .arg("connect_vsock")
                    .arg(port.to_string())
                    .arg(socket.as_ref())
                    .status()?;
                if status.success() {
                    Ok(VmResponse::Ok)
                } else {
                    Err(Error::other(format!(
                        "crosvm connect_vsock failed with status {}",
                        status
                    )))
                }
            }
            VmRequest::BalloonCommand(BalloonControlCommand::Adjust {
                num_bytes,
                wait_for_success,
            }) => {
                let mut cmd = Command::new(crosvm_exe());
                cmd.arg("balloon").arg(num_bytes.to_string()).arg(socket.as_ref());
                if *wait_for_success {
                    cmd.arg("--wait");
                }
                let status = cmd.status()?;
                if status.success() {
                    Ok(VmResponse::Ok)
                } else {
                    Err(Error::other(format!(
                        "crosvm balloon failed with status {}",
                        status
                    )))
                }
            }
            VmRequest::BalloonCommand(BalloonControlCommand::Stats {}) => {
                let output = Command::new(crosvm_exe())
                    .arg("balloon_stats")
                    .arg(socket.as_ref())
                    .output()?;
                if !output.status.success() {
                    return Ok(VmResponse::Err(SysError(libc::ENOTSUP)));
                }
                let stdout = String::from_utf8_lossy(&output.stdout);
                let trimmed = stdout.trim();
                if trimmed.is_empty() {
                    return Ok(VmResponse::Err(SysError(libc::ENOTSUP)));
                }
                let json: Value = match serde_json::from_str(trimmed) {
                    Ok(v) => v,
                    Err(_) => return Ok(VmResponse::Err(SysError(libc::ENOTSUP))),
                };
                if let Some(actual) = find_balloon_actual(&json) {
                    Ok(VmResponse::BalloonStats {
                        stats: (),
                        balloon_actual: actual,
                    })
                } else {
                    Ok(VmResponse::Err(SysError(libc::ENOTSUP)))
                }
            }
        }
    }
}
