//! examples/lab4_example

#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use core::panic::PanicInfo;
use syncrim_clic_rt as _;
use riscv_rt as _;
//static mut COUNTER:u32 = 0x1337;

#[rtic::app(device = clic, dispatchers = [Interrupt1, Interrupt2, Interrupt3])]
mod app{
    #[shared]
    struct Shared{}

    #[local]
    struct Local{}
    
    #[init]
    fn init(_:init::Context) -> (Shared, Local) {
        bar::spawn().ok();
        (Shared{},Local{})
    }

    #[idle]
    fn idle(_:idle::Context) ->! {
        loop{}
    }
    #[task(priority=1)]
    async fn foo(_:foo::Context) {

    }
    #[task(priority=2)]
    async fn bar(_:bar::Context) {
        foo::spawn().ok();
        baz::spawn().ok();
    }
    #[task(priority=3)]
    async fn baz(_:baz::Context) {

    }
}
#[panic_handler]
unsafe fn _panic(_:&PanicInfo)->!{
    loop{}
}
