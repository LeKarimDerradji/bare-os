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

### Panic
Because we disabled the standard library, when the program crash, there's no panic handler. 
So we need to define it ourselves :

```rust
// In main.rs
// Create a panic_handler 
#[panic_handler]
// This function is called on panic 
fn panic(_info: &PanicInfo) -> ! {
    //What happen on panic
}
```

The `#[panic_handler]` attribute defines the function that the compiler should invoke when a panic occurs.

The `PanicInfo` parameter contains the file path and the line where the panic occurs.
This function should never return, so we mark it as a diverging function, with the symbol `!`.

### Disabling Unwinding

Now we have to disable unwinding, in Rust it is used to run the destructors of all live stack variables in case of a panic.
But it requires OS specific libraries to function, so we need to disable it. 

The best way to do that, is to edit `Cargo.toml`.

```rust
// In Cargo.toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

This sets the panic strategy to abort for both the `dev` profile (used for `cargo build`) and the release profile (used for `cargo build --release`).

### Start

Every language has a runtime system, that specifies an execution model, as well as setting up and managing the stack, the heap, and including other features such as garbage collection, threads or other dynamic features built into the language.

In Rust, execution starts at a C runtime library called `crt0` to manage the process stack, create space for local variables... then it invoke the `start`entry point in our Rust program. 

Which finally call the `main`function of our program.

Since we don't use the standard library, we need to overwrite the `start`language item ourselve, implementing it will still require `crt0`to invoke it. 

To instruct the Rust compiler to not use the normal entry point chain. 
We add `#![no_main]`to the top our program. 

We can now overwrite the operating system entry point with a custom `_start`function. 

```rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
```

Here, we need to use `#[no_mangle]` to disable name mangling, so that the Rust compiler really outputs this function with `_start` as a name. 

We also mark the function as `extern "C"`to tell the compiler that this function should be called using the C calling convention, so it gets invoked by most system when it boots. 

The function is also diverging, cause it's a freestanding binary, therefore it should not return a main entry point. 

## From freestanding binary to bootable disk image. 

Now that we've created a minimal kernel, it's time to improve it, the next step is to make the binary bootable, after the booting process, our kernel should print something to the screen. 

This part will talk about computer boot sequence, an overview of the Rust toolchain, cutomizing builds with Rust and Cargo, dependencies, creating a bootimage, and printing characters to the screen by writting on the VGA text buffer at physical memory address 0xB8000.

### The Boot Process 

When you turn on a computer, it loads the BIOS from some special flash memory located on the motherboard. The BIOS runs self-test and initialization routines of the hardware, then it looks for bootable disks. If it finds one, control is transferred to its bootloader, which is a 512-byte portion of executable code stored at the disk’s beginning. Most bootloaders are larger than 512 bytes, so bootloaders are commonly split into a small first stage, which fits into 512 bytes, and a second stage, which is subsequently loaded by the first stage.

The bootloader has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from the 16-bit real mode first to the 32-bit protected mode, and then to the 64-bit long mode, where 64-bit registers and the complete main memory are available. Its third job is to query certain information (such as a memory map) from the BIOS and pass it to the OS kernel.

### Bootimage

Because writting a bootloader is not part of this project, we're using https://github.com/rust-osdev/bootimage to prepend a bootloader to our minimal Kernel. 

### Rust Nightly 

Rust has three release channel: stable, beta, and nightly.
The nightly channel allows the use of experimental features. 
To use the nighlty channel, we just have to type `rustup override set nightly`into the terminal.  

### Target Specs

With Cargo, the rust package manager, we can specify the target systems we would like the compiler to build for. 
Rust allow us to specify our custom target with a `.json`file that we simply put in the project's root, with all the specifications as key-value pairs. 

```json
{
    "llvm-target": "x86_64-unknown-none",
    "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": "32",
    "os": "none",
    "executables": true,
    "linker-flavor": "ld.lld",
    "linker": "rust-lld",
    "panic-strategy": "abort",
    "disable-redzone": true,
    "features": "-mmx,-sse,+soft-float"
}
```



### Building the Kernel

### Printing on Screen

### Running the Kernel 

### Booting the Kernel












