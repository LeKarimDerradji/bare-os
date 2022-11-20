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



