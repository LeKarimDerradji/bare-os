#![no_std] // Disable Rust standard library
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![test_runner(bare_metal_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bare_metal_os::println;
use core::panic::PanicInfo; // Import PanicInfo from panic in the core lib
                            // Disable name mangling on this function
                            // This function will be called on start as an entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    bare_metal_os::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();
    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bare_metal_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
