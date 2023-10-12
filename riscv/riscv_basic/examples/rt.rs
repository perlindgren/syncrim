#![no_std]
#![no_main]
use riscv_rt::entry;
use core::panic::PanicInfo;
use clic::interrupt;
#[entry]
unsafe fn main()->!{
    loop{}
}

#[interrupt]
unsafe fn GPIOA(){

}

#[panic_handler]
fn _panic(_:&PanicInfo)->!{
    loop{}
}
