#![no_std]
#![no_main]
use clic::interrupt;
use core::panic::PanicInfo;
use riscv_rt::entry;
#[entry]
unsafe fn main() -> ! {
    loop {}
}

#[interrupt]
unsafe fn GPIOA() {}

#[panic_handler]
fn _panic(_: &PanicInfo) -> ! {
    loop {}
}
