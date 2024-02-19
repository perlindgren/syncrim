#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use core::panic::PanicInfo;
use riscv_rt as _;
use syncrim_clic_rt as _;
//static mut COUNTER:u32 = 0x1337;

#[rtic::app(device = syncrim_pac)]
mod app {
   // use embedded_hal::digital::StatefulOutputPin;
   // use syncrim_hal::gpio::{Output, Pin, Pins};
   // use syncrim_hal::mtime;
    #[shared]
    struct Shared {
        low_prio_r: u32,
        high_prio_r: u32,
    }

   // type LED = Pin<Output>;
    #[local]
    struct Local {
     //   led: LED,
     //   mtime: mtime::MTIME,
    }

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local) {
       // let peripherals = cx.device;
       // let g = peripherals.GPIO;
      //  let m = peripherals.MTIME;
     //   let pins = Pins::new(g);
      //  let mut mtime = mtime::MTIME::new(m);
      //  mtime.set_compare_in(50_000);
      //  let led = pins.pin2.into_output();
        let low_prio_r = 0;
        let high_prio_r = 0;
        rtic::export::pend(clic::Interrupt2);
        //(Shared {low_prio_r, high_prio_r}, Local {led, mtime})
        (Shared {low_prio_r, high_prio_r}, Local {})

    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop{}
    }


    #[task(binds = Interrupt2, priority = 2, shared=[low_prio_r, high_prio_r])]
    fn i2(mut cx: i2::Context){
        cx.shared.high_prio_r.lock(|high_prio_r|{
            cx.shared.low_prio_r.lock(|low_prio_r|{
                rtic::export::pend(clic::Interrupt4);
                rtic::export::pend(clic::Interrupt5);
                *low_prio_r += 1;
                *high_prio_r += 1;
            });
            *high_prio_r += 1;
        });
        cx.shared.low_prio_r.lock(|low_prio_r|{
            rtic::export::pend(clic::Interrupt3);
            rtic::export::pend(clic::Interrupt4);
            *low_prio_r += 1;
        })
    }

    #[task(binds = Interrupt3, priority = 3, shared=[low_prio_r])]
    fn i3(mut cx: i3::Context){
        cx.shared.low_prio_r.lock(|low_prio_r|{
            *low_prio_r+=1;
        });
    }
    #[task(binds = Interrupt4, priority = 4, shared=[high_prio_r])]
    fn i4(mut cx: i4::Context){
        cx.shared.high_prio_r.lock(|high_prio_r|{
            *high_prio_r += 1;
        });
    }
    #[task(binds = Interrupt5, priority = 5)]
    fn i5(_: i5::Context){}

   // #[task(binds = MTIME, local = [led, mtime], priority = 1)]
   // fn timer_interrupt(cx: timer_interrupt::Context) {
   //     cx.local.mtime.set_compare_in(50_000); 
   //     let _ = cx.local.led.toggle();
   // }
}
#[panic_handler]
unsafe fn _panic(_: &PanicInfo) -> ! {
    loop {}
}
