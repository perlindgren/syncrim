#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use core::panic::PanicInfo;
use riscv_rt as _;
use syncrim_clic_rt as _;
//static mut COUNTER:u32 = 0x1337;

#[rtic::app(device = syncrim_pac)]
mod app {
  use embedded_hal::digital::StatefulOutputPin;
  use syncrim_hal::gpio::{Output, Pin, Pins};
  type LED = Pin<Output>;
  #[shared]
  struct Shared {
    led: LED,
    resource: u32,
  }

  #[local]
  struct Local {}

  #[init]
  fn init(cx: init::Context) -> (Shared, Local) {
    let peripherals = cx.device;
    let g = peripherals.GPIO;
    let pins = Pins::new(g);
    let led = pins.pin2.into_output();
    let resource = 0;
    rtic::export::pend(clic::Interrupt2);
    rtic::export::pend(clic::Interrupt1);
    //(Shared {low_prio_r, resource}, Local {led, mtime})
    (Shared { led, resource }, Local {})
  }

  #[idle]
  fn idle(_: idle::Context) -> ! {
    loop {}
  }

  #[task(binds = Interrupt1, priority = 1, shared = [led])]
  fn i1(mut cx: i1::Context) {
    cx.shared.led.lock(|led| {
      let _ = led.toggle();
    });
  }
  #[task(binds = Interrupt2, priority = 2, shared=[led, resource])]
  fn i2(mut cx: i2::Context) {
    cx.shared.resource.lock(|resource| {
      cx.shared.led.lock(|led| {
        rtic::export::pend(clic::Interrupt4);
        let _ = led.toggle();
        *resource += 1;
      });
      *resource += 1;
    });
    cx.shared.led.lock(|led| {
      rtic::export::pend(clic::Interrupt3);
      rtic::export::pend(clic::Interrupt4);
      let _ = led.toggle();
    })
  }

  #[task(binds = Interrupt3, priority = 3, shared=[led])]
  fn i3(mut cx: i3::Context) {
    cx.shared.led.lock(|led| {
      let _ = led.toggle();
    });
  }
  #[task(binds = Interrupt4, priority = 4, shared=[resource])]
  fn i4(mut cx: i4::Context) {
    cx.shared.resource.lock(|resource| {
      *resource += 1;
    });
  }
}
#[panic_handler]
unsafe fn _panic(_: &PanicInfo) -> ! {
  loop {}
}
