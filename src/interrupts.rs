#![feature(abi_x86_interrupt)]

use crate::println;
use lazy_static::lazy_static;
use x86_64::{structures::idt::{InterruptDescriptorTable, InterruptStackFrame}, instructions::interrupts};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    interrupts::init_idt();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}