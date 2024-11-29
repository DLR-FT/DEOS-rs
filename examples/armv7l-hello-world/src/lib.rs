//! For reference, check out <https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html>
#![no_main]
#![no_std]

use core::panic::PanicInfo;

/// This module contains the bindings to the C API/ABI of DEOS
pub mod bindings {
    #![allow(clippy::redundant_static_lifetimes)]
    #![allow(dead_code)]
    #![allow(missing_docs)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(rustdoc::broken_intra_doc_links)]
    core::include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod fmt;

#[panic_handler]
pub fn panic(panic: &PanicInfo<'_>) -> ! {
    use core::fmt::Write;

    // log the issue to the system log
    // process log is not feasible, as the process log is deleted when the process is deleted
    writeln!(ProcessEventWriter, "{panic}").unwrap();
    writeln!(SystemEventWriter, "{panic}").unwrap();

    // delete this process
    unsafe { bindings::deleteProcess(bindings::currentProcessHandle()) };

    // unreacheable, but if reached, well, still do nothing!
    loop {}
}

// Allows to write system events
pub struct SystemEventWriter;

impl core::fmt::Write for SystemEventWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let str_bytes = s.as_bytes();

        // 4 KiB Buffer
        let mut buf = [0u8; 0x1000];
        let buf_len = buf.len();

        // Split into 4 KiB - 1 usable bytes and one remaining nullbyte
        let (usable_buf, _remaining_null_byte) = &mut buf.split_at_mut(buf_len - 1);

        // log everything, split into chunks if the message is too big
        for chunk in str_bytes.chunks(usable_buf.len()) {
            usable_buf[0..chunk.len()].copy_from_slice(chunk);

            if chunk.len() < usable_buf.len() {
                usable_buf[chunk.len()] = 0;
            }

            unsafe {
                bindings::logSystemEvent(usable_buf.as_ptr() as _);
            }
        }
        Ok(())
    }
}

// Allows to write system events
pub struct ProcessEventWriter;

impl core::fmt::Write for ProcessEventWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let str_bytes = s.as_bytes();

        // 4 KiB Buffer
        let mut buf = [0u8; 0x1000];
        let buf_len = buf.len();

        // Split into 4 KiB - 1 usable bytes and one remaining nullbyte
        let (usable_buf, _remaining_null_byte) = &mut buf.split_at_mut(buf_len - 1);

        // log everything, split into chunks if the message is too big
        for chunk in str_bytes.chunks(usable_buf.len()) {
            usable_buf[0..chunk.len()].copy_from_slice(chunk);

            if chunk.len() < usable_buf.len() {
                usable_buf[chunk.len()] = 0;
            }

            unsafe {
                bindings::logProcessEvent(usable_buf.as_ptr() as _);
            }
        }
        Ok(())
    }
}
