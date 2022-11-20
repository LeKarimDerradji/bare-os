#![no_std] // Disable Rust standard library
#![no_main] // Disable all Rust-level entry points.

use core::panic::PanicInfo; // Import PanicInfo from panic in the core lib

#[no_mangle] // Disable name mangling on this function
// This function will be called on start as an entry point
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

// Create a panic_handler 
#[panic_handler]
// This function is called on panic 
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
