#![feature(abi_x86_interrupt)]

use x86_64::structures::{self, idt::InterruptDescriptorTable}:idt::InteruptDescriptorTable;


pub fn init_idt() {
    let mut idt = InterruptDescriptorTable::new();
}
