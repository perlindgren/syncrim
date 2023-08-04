#![no_std]
#![no_main]
use core::arch::global_asm;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

global_asm! {
    include_str!("../asm.s")
}
