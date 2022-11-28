#![no_std] // Disable Rust standard library
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;
use core::panic::PanicInfo; // Import PanicInfo from panic in the core lib



#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}


// Disable name mangling on this function
// This function will be called on start as an entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();
    
    loop {}
}

// Create a panic_handler
#[panic_handler]
// This function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
