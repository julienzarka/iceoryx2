// Copyright (c) 2026 Contributors to the Eclipse Foundation
//
// See the NOTICE file(s) distributed with this work for additional
// information regarding copyright ownership.
//
// This program and the accompanying materials are made available under the
// terms of the Apache Software License 2.0 which is available at
// https://www.apache.org/licenses/LICENSE-2.0, or the MIT license
// which is available at https://opensource.org/licenses/MIT.
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Plain wrapper struct: an externally-allocated buffer identified by an
//! `OwnedFd` and a known length. Used as the configuration handle for
//! [`crate::shm::FdBackedSharedMemory::from_owned_fd`].
//!
//! Compared to a tuple `(OwnedFd, usize)`, this struct keeps the field names
//! self-documenting and makes future extensions (alignment, allocator-id, …)
//! source-compatible.

use std::os::fd::{AsFd as _, BorrowedFd, OwnedFd};

/// An externally-allocated buffer identified by an fd and a byte length.
///
/// Fields are private to preserve the invariant that `len` matches the fd's
/// actual allocated size after construction.
#[derive(Debug)]
pub struct ExternalFdBuffer {
    fd: OwnedFd,
    len: usize,
}

impl ExternalFdBuffer {
    /// Wraps an externally-allocated fd of size `len` bytes.
    #[must_use]
    pub fn new(fd: OwnedFd, len: usize) -> Self {
        Self { fd, len }
    }

    /// Returns the size of the buffer in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the buffer has zero length.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Borrows the underlying file descriptor.
    pub fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }

    /// Consumes `self` and returns the underlying `OwnedFd`.
    pub fn into_fd(self) -> OwnedFd {
        self.fd
    }
}
