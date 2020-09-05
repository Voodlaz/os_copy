#![no_std]
pub mod interrupts;
extern crate rlibc;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
}
