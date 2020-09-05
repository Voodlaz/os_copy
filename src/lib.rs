#![no_std]
pub mod interrupts;
extern crate rlibc;

pub fn init() {
    interrupts::init_idt();
}
