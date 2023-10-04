// SPDX-License-Identifier: GPL-2.0

//! Rust character device sample.
use kernel::macros;
use core::sync::atomic::{AtomicBool, AtomicU64};
use kernel::prelude::*;
use kernel::sync::{smutex::Mutex};
use kernel::pr_info;

const BUFFER_SIZE: usize = 1024 * 512;

module! {
    type: RustChrdev,
    name: "mymem",
    author: "Rust for Linux Contributors",
    description: "Rust character device sample",
    license: "GPL",
}
static SHARED_STATE: Mutex<[u8; BUFFER_SIZE]> = Mutex::new([0u8; BUFFER_SIZE]);

#[no_mangle]
pub extern "C" fn mymem_read(outbuf: &mut [u8], offset: usize) -> usize {
    let lock = SHARED_STATE.lock();
    
    // Ensure that the offset is within bounds.
    if offset >= BUFFER_SIZE {
        return usize::MAX;
    }

    let bytes_to_read = core::cmp::min(outbuf.len(), BUFFER_SIZE - offset);
    outbuf[..bytes_to_read].copy_from_slice(&lock[offset..offset + bytes_to_read]);
    
    bytes_to_read
}

#[no_mangle]
pub extern "C" fn mymem_write(inbuf: &[u8], offset: usize) -> usize {
    let mut lock = SHARED_STATE.lock();
    
    // Ensure that the offset is within bounds.
    if offset >= BUFFER_SIZE {
        return usize::MAX;
    }

    let bytes_to_write = core::cmp::min(inbuf.len(), BUFFER_SIZE - offset);
    lock[offset..offset + bytes_to_write].copy_from_slice(&inbuf[..bytes_to_write]);
    
    bytes_to_write
}

struct RustChrdev;

impl kernel::Module for RustChrdev {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust character device sample (init)\n");
        pr_info!("Exported functions from mymem:\n");
        pr_info!(" - mymem_read\n");
        pr_info!(" - mymem_write\n");
        Ok(RustChrdev)
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}