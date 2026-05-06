// SPDX-License-Identifier: Apache-2.0 OR MIT
#![cfg(target_os = "linux")]
#![allow(dead_code)] // Some tests use a subset of these helpers.

use std::os::fd::{FromRawFd as _, OwnedFd};
use std::time::Duration;

/// Create a zero-length memfd (no ftruncate).
///
/// Used by tests that pass the fd through the UDS channel without mapping it.
pub fn memfd(name: &core::ffi::CStr) -> OwnedFd {
    // SAFETY: memfd_create is a standard Linux syscall; name is a valid CStr.
    let raw = unsafe { libc::memfd_create(name.as_ptr(), 0) };
    assert!(raw >= 0, "memfd_create failed");
    // SAFETY: raw is a valid, owned file descriptor returned by memfd_create.
    unsafe { OwnedFd::from_raw_fd(raw) }
}

/// Create a memfd sized to `len` bytes via ftruncate.
///
/// Used by tests that mmap or stat the fd after transmission.
pub fn memfd_sized(name: &core::ffi::CStr, len: i64) -> OwnedFd {
    let fd = memfd(name);
    // SAFETY: fd is a valid fd returned by memfd_create; ftruncate sets its size.
    let rc = unsafe { libc::ftruncate(std::os::fd::AsRawFd::as_raw_fd(&fd), len) };
    assert_eq!(rc, 0, "ftruncate({len}) failed");
    fd
}

/// Poll until `f()` returns `true`, or until `deadline` elapses (last check included).
///
/// Synchronous test-only helper; the 2 ms sleep is intentional — this is never
/// called from async or production code.
pub fn wait_until<F: FnMut() -> bool>(deadline: Duration, mut f: F) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < deadline {
        if f() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    f()
}

/// Generate a unique UDS socket path for a test identified by `tag`.
///
/// Includes the process PID to avoid collisions between parallel test runs.
pub fn unique_socket_path(tag: &str) -> String {
    format!("/tmp/iox2-conntest-{}-{}.sock", std::process::id(), tag)
}

/// Generate a unique iceoryx2 service name for a test identified by `tag`.
///
/// Includes the process PID to avoid collisions between parallel test runs.
pub fn unique_service_name(tag: &str) -> String {
    format!("dmabuf/test/{tag}/{}", std::process::id())
}
