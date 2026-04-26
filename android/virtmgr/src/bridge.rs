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

//! Unified TCP-to-vsock bridge for ADB and other host-guest port forwarding.
//!
//! Bridges TCP connections received on a host port to a guest vsock port.
//! The vsock connection is provided by the caller, so this module contains
//! no platform-dependent code — it is used by both `crosvm_unix.rs` and
//! `crosvm_windows.rs`.

use anyhow::Context;
use log::error;
use std::fs::File;
use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Per-port bridge handle. Dropping or calling [`stop`](BridgeHandle::stop)
/// signals the listener loop to terminate.
pub struct BridgeHandle {
    _running: Arc<AtomicBool>,
}

/// Start a TCP listener on `127.0.0.1:<host_port>`.
///
/// For each accepted TCP connection, `on_accept` is called with the stream.
/// The listener loop runs until [`BridgeHandle::stop`] is called.
pub fn start_bridge<F>(host_port: u16, cid: u32, guest_port: u32, on_accept: F) -> io::Result<BridgeHandle>
where
    F: Fn(TcpStream) + Send + 'static,
{
    let listener = TcpListener::bind(("127.0.0.1", host_port))?;
    listener.set_nonblocking(true)?;

    let running = Arc::new(AtomicBool::new(true));
    let flag = Arc::clone(&running);

    thread::Builder::new()
        .name(format!("bridge-{host_port}"))
        .spawn(move || {
            while flag.load(Ordering::Acquire) {
                match listener.accept() {
                    Ok((tcp, remote)) => {
                        let _ = remote;
                        on_accept(tcp);
                    }
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(err) => {
                        error!("[bridge cid={cid}] 127.0.0.1:{host_port} <-> vsock:{guest_port} — listener error: {err}");
                        break;
                    }
                }
            }
        })
        .map(|_| BridgeHandle { _running: running })
        .with_context(|| format!("failed to spawn bridge thread for localhost:{host_port}"))
        .map_err(|e: anyhow::Error| io::Error::new(io::ErrorKind::Other, e))
}

impl BridgeHandle {
    /// Stop the bridge listener. Already-established connections continue
    /// until they complete or the guest disconnects.
    pub fn stop(&self) {
        self._running.store(false, Ordering::Release);
    }
}

/// Bridge a single TCP connection to a guest vsock file using bidirectional
/// `io::copy` in two threads. Blocks until both copy directions finish.
pub fn bridge_connection(tcp: TcpStream, guest: File) -> io::Result<()> {
    let mut guest_reader = guest.try_clone()?;
    let mut guest_writer = guest;
    let mut tcp_reader = tcp.try_clone()?;
    let mut tcp_writer = tcp;

    let host_to_guest = thread::spawn(move || -> io::Result<u64> {
        let copied = io::copy(&mut tcp_reader, &mut guest_writer)?;
        let _ = guest_writer.flush();
        Ok(copied)
    });
    let guest_to_host = thread::spawn(move || -> io::Result<u64> {
        let copied = io::copy(&mut guest_reader, &mut tcp_writer)?;
        let _ = tcp_writer.shutdown(std::net::Shutdown::Write)?;
        Ok(copied)
    });

    let _ = host_to_guest
        .join()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "host_to_guest bridge thread panicked"))?
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let _ = guest_to_host
        .join()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "guest_to_host bridge thread panicked"))?
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}
