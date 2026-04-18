#[cfg(unix)]
#[path = "crosvm_unix.rs"]
mod crosvm_unix;
#[cfg(unix)]
pub use crosvm_unix::*;

#[cfg(windows)]
#[path = "crosvm_windows.rs"]
mod crosvm_windows;
#[cfg(windows)]
pub use crosvm_windows::*;
