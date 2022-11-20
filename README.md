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




