#![no_std] // Disable Rust standard library
#![no_main] // Disable all Rust-level entry points.
mod vga_buffer;
use core::panic::PanicInfo; // Import PanicInfo from panic in the core lib


static HELLO: &[u8] = b"Hello World!";

// Disable name mangling on this function
// This function will be called on start as an entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// Create a panic_handler
#[panic_handler]
// This function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
