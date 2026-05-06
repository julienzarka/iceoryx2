// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Shared error type for the typed DMA-BUF publisher/subscriber layer.
//!
//! Both [`crate::dmabuf_publisher::DmaBufPublisher`] and
//! [`crate::dmabuf_subscriber::DmaBufSubscriber`] return this error.

use crate::service_error::ServiceError;

/// Errors returned by [`crate::dmabuf_publisher::DmaBufPublisher`] and
/// [`crate::dmabuf_subscriber::DmaBufSubscriber`].
#[derive(Debug)]
#[non_exhaustive]
pub enum DmaBufError {
    /// An underlying [`ServiceError`] from service creation or port operations.
    Service(ServiceError),
    /// `fcntl(F_DUPFD_CLOEXEC)`, `fstat`, or type-conversion on the DMA-BUF fd failed.
    FdDup(std::io::Error),
}

impl core::fmt::Display for DmaBufError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Service(e) => write!(f, "service error: {e}"),
            Self::FdDup(e) => write!(f, "fd dup/stat failed: {e}"),
        }
    }
}

impl core::error::Error for DmaBufError {}

impl From<ServiceError> for DmaBufError {
    fn from(e: ServiceError) -> Self {
        Self::Service(e)
    }
}
