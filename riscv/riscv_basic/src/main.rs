// #![no_std]
// #![no_main]

// use core::panic::PanicInfo;
// use riscv::register::mstatus;
// use riscv_rt::entry;
// use clic::register::mintthresh;
// use clic::peripherals::{CLIC, Peripherals};
// use clic::interrupt::Interrupts;
// use syncrim_clic_rt as _;
// #[entry]
// unsafe fn main() -> ! {
//     mintthresh::write(0); //prio threshold = 0
//     mstatus::set_mie();        //enable interrupts globally
//     let mut clic = Peripherals::take().unwrap().CLIC;
//     CLIC::unmask(Interrupts::Interrupt0);
//     clic.set_priority(Interrupts::Interrupt0, 2);
//     CLIC::pend(Interrupts::Interrupt0);
//     loop {}
// }
// #[no_mangle]
// unsafe fn _interrupt0(){
//     CLIC::unpend(Interrupts::Interrupt0);
// }

// #[panic_handler]
// fn panic(_: &PanicInfo) -> ! {
//     loop {}
// }

#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use core::panic::PanicInfo;
use clic::Interrupt;
use riscv_rt::entry;
use syncrim_clic_rt as _;
use riscv::interrupt;

#[entry]
fn main() -> ! {
    unsafe{interrupt::enable();}
    let mut peripherals = unsafe{clic::peripherals::Peripherals::steal()};
    unsafe{
        peripherals.CLIC.unmask(Interrupt::TIM1);
        peripherals.CLIC.set_priority(Interrupt::TIM1, 1);
        peripherals.CLIC.unmask(Interrupt::Interrupt2);
        peripherals.CLIC.set_priority(Interrupt::Interrupt2, 2);
    
    }

    let mut ANTIQ_BASE = 0x5010 as *mut u32;
    unsafe{
        core::ptr::write_volatile(ANTIQ_BASE, 0xA01);
        core::ptr::write_volatile(ANTIQ_BASE, 0xF02);
    }


    unsafe{core::arch::asm!("
    nop
    nop
    nop
    nop
    nop
    nop
    ")}; //marker
    loop{}
   // panic!();
}

#[no_mangle]
fn _interrupt1(){
    loop{}
}

#[no_mangle]
fn _interrupt2(){
    unsafe{
        core::arch::asm!("
        nop
        nop
        nop
        nop

        ");
    }
}
/*
#[rtic::app(device = clic, peripherals = false, dispatchers=[GPIOA, TIM1])]
mod app {
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        foo::spawn().unwrap();
        (Shared {}, Local {})
    }

    #[task(priority = 2)]
    async fn foo(_: foo::Context) {
        bar::spawn().unwrap();
        loop{}
    }

    #[task(priority = 3)]
    async fn bar(_: bar::Context) {

    }
}
*/
#[panic_handler]
fn panic(_:&PanicInfo)->!{
    loop{}
}
