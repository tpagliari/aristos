#![no_std] // build for bare-metal: don't link the standard library
#![no_main] // provide our own entry point instead of Rust's runtime

mod vga_buffer;

use vga_buffer::WRITER;
use core::panic::PanicInfo;

/** Entry point the bootloader or runtime will jump to;
must have an unmangled symbol named `_start`. */
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    write!(WRITER.lock(), "Welcome to AristOS,\nan os from the ancient Greece.\n\n").unwrap();
    write!(WRITER.lock(), "Take these numbers: {} and {}", 86, 128/2).unwrap();
    
    // Prevent returning; on bare metal we should not fall back to any runtime
    loop {}
}

/** Minimal panic handler required when building with #![no_std] */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}