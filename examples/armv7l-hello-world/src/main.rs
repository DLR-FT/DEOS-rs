//! For reference, check out https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#![no_main]
#![no_std]

use deos_api::bindings::{
     getenv, open, systemTickPointer, waitUntilNextPeriod, O_WRONLY,
};

use core::fmt::Write;
use deos_api::fmt::Fd;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    /* Grab the system tick pointer. */
    let system_tick_ptr = unsafe { systemTickPointer() };

    /* Get the path to where we want to direct output. You can easily add or change
    environment variables in the hello-world.pd.xml file. */
    let path = unsafe { getenv(c"OUTPUT_FILE".as_ptr()) };
    if path.is_null() {
        panic!("*** Failed to read 'OUTPUT_FILE' environment variable ***");
    }

    /* If file descriptor (fd) is -1, the file failed to open. If this happens,
    ensure that you are using the diagnostic (debug variant) of vfile. */
    let fd = unsafe { open(path, O_WRONLY as _) };
    if fd == -1 {
        panic!("*** Failed to open {path:?} ***");
    }

    /* The main thread goes into an infinite loop. */
    loop {
        /* Write the system tick value to the specified file. */
        // unsafe {
        //     fdprintf(
        //         fd,
        //         c"Hello, Rusty World! System Tick = %u\n".as_ptr(),
        //         core::ptr::read_volatile(system_tick_ptr),
        //     )
        // };
        writeln!(Fd(fd), "Now some proper Rust formatting {:b}", unsafe {
            core::ptr::read_volatile(system_tick_ptr)
        })
        .unwrap();

        /* Yield the CPU and wait until the next period to run again. */
        unsafe { waitUntilNextPeriod() };
    }
}
