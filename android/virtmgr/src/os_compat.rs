// Copyright 2026, The Android Open Source Project
//
// On Unix: re-export standard `std::os::unix` IO / process extensions.
// On Windows: minimal FD shims so sources compile; some paths are stubs for portability.

#[cfg(unix)]
pub mod imp {
    pub use std::os::unix::fs::FileExt;
    pub use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
    pub use std::os::unix::process::ExitStatusExt;
    pub use std::os::unix::raw::pid_t;
}

#[cfg(unix)]
pub use imp::*;

#[cfg(windows)]
pub mod imp {
    /// MinGW libc exposes process IDs as `int` (see POSIX `pid_t` on Windows ports).
    pub type pid_t = libc::c_int;

    /// CRT file descriptor for MinGW; used where Android code expects `AsRawFd` on `File`.
    pub trait AsRawFd {
        fn as_raw_fd(&self) -> libc::c_int;
    }

    impl AsRawFd for std::fs::File {
        fn as_raw_fd(&self) -> libc::c_int {
            use std::os::windows::io::AsRawHandle;
            let h = self.as_raw_handle();
            // Associate a CRT `int` fd with this Windows handle (MSVCRT / MinGW).
            unsafe { libc::open_osfhandle(h as libc::intptr_t, libc::O_RDWR | libc::O_BINARY) }
        }
    }
}

#[cfg(windows)]
pub use imp::{pid_t, AsRawFd};
