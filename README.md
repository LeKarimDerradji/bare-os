# An Operating-System written in Rust

To deepen my understanding of computer science and Rust, 
I am building an operating system. 

## The Kernel 

The first step is to create a Kernel, for that we need to remove the STD-LIB, 
The Rust standard library is very useful and comes with a lot of features, 
but these features requires OS abstractions to work. 
So in order for that binary to be executed on bare-metal, without the need of an OS, 
we need to diseable the standard library. 

For that, we declare `!#[no_std]` at the begining of our program. 

## Start and Panic

Because we disabled the standard library, when the program crash, there's no panic handler. 
So we need to define it ourselves :

`// in main.rs

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Behavior on panic
}`

The `#[panic_handler]` attribute defines the function that the compiler should invoke when a panic occurs.

The `PanicInfo` parameter contains the file path and the line where the panic occurs.
This function should never return, so we mark it as a diverging function, with the symbol `!`



