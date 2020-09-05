#![no_std]
#![no_main]

extern crate rlibc;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    os::init();
    x86_64::instructions::interrupts::int3();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
