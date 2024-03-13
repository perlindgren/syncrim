#![no_std]
#![no_main]
use core::panic::PanicInfo;
use hippomenes_core::gpio::Pin0;
use hippomenes_core::mstatus::*;
use hippomenes_core::{interrupt1, interrupt2};
use hippomenes_rt::entry;
#[entry]
unsafe fn main() -> ! {
    MIE::set();
    interrupt1::Enabled::set();
    interrupt1::Priority::set(2);
    interrupt2::Enabled::set();
    interrupt2::Priority::set(3);
    interrupt1::Pending::set();
    loop {}
}

#[no_mangle]
fn Interrupt1() {
    interrupt2::Pending::set();
    Pin0::clear();
}

#[no_mangle]
fn Interrupt2() {
    interrupt1::Pending::set();
    Pin0::set();
}

#[panic_handler]
fn p(_: &PanicInfo) -> ! {
    loop {}
}
