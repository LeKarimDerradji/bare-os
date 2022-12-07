#![no_std] // Disable Rust standard library
#![no_main] // Disable all Rust-level entry points.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo; // Import PanicInfo from panic in the core lib
use bare_metal_os::println;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        bare_metal_os::serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        bare_metal_os::serial_println!("[ok]");
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    bare_metal_os::serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bare_metal_os::serial_println!("[failed]\n");
    bare_metal_os::serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
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
#[cfg(not(test))]
#[panic_handler]
// This function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}
