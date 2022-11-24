#![no_std] // Disable Rust standard library
#![no_main] // Disable all Rust-level entry points.
mod vga_buffer;
use core::panic::PanicInfo; // Import PanicInfo from panic in the core lib

// Disable name mangling on this function
// This function will be called on start as an entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    loop {}
}

// Create a panic_handler
#[panic_handler]
// This function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
