#![no_std]
#![no_main]

use core::panic::PanicInfo;
use hippomenes_core::Peripherals;
use hippomenes_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
