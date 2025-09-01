#![no_std] // build for bare-metal: don't link the standard library
#![no_main] // provide our own entry point instead of Rust's runtime

use core::panic::PanicInfo;

// byte slice so we can copy raw bytes directly into VGA memory
static HELLO: &[u8] = b"Hello World!";

/** Entry point the bootloader or runtime will jump to;
must have an unmangled symbol named `_start`. */
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    // VGA text buffer base address on legacy x86;
    // write bytes here to display characters on screen.
    let vga_buffer: *mut u8 = 0xb8000 as *mut u8;
    
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    // Prevent returning; on bare metal we should not fall back to any runtime
    loop {}
}

/** Minimal panic handler required when building with #![no_std] */
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}