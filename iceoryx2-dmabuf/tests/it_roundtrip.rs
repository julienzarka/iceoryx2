// SPDX-License-Identifier: Apache-2.0 OR MIT
#![cfg(all(target_os = "linux", feature = "dma-buf"))]

mod common;
use common::*;

use iceoryx2_dmabuf::{DmaBufPublisher, DmaBufSubscriber};
use std::time::Duration;

#[test]
fn typed_publish_receive_via_memfd_wrapped_in_dmabuf() {
    let mut pubr =
        DmaBufPublisher::<u64>::create("dmabuf/test/typed-rt").expect("publisher create");
    let mut subr =
        DmaBufSubscriber::<u64>::create("dmabuf/test/typed-rt").expect("subscriber create");

    // Wait for the UDS fd-channel handshake.
    std::thread::sleep(Duration::from_millis(50));

    let fd = memfd_sized(c"rt", 4096);
    let buf: dma_buf::DmaBuf = fd.into();
    pubr.publish(42u64, &buf).expect("publish");

    let recvd = wait_until(Duration::from_millis(500), || {
        matches!(subr.receive(), Ok(Some((42, _))))
    });
    assert!(recvd, "did not receive typed sample within deadline");
}
