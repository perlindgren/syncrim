#![no_main]
#![no_std]
use core::panic::PanicInfo;
use hippomenes_rt as _;
#[rtic::app(device = hippomenes_core)]
mod app {
  use hippomenes_core::{interrupt1, interrupt2};
  #[shared]
  struct Shared {
    r: u8,
    l: u8,
  }

  #[local]
  struct Local {}

  #[init]
  fn init(cx: init::Context) -> (Shared, Local) {
    rtic::export::pend(interrupt1::Interrupt1);
    rtic::export::pend(interrupt2::Interrupt2);
    let r = 9;
    let l = 10;
    (Shared { r, l }, Local {})
  }

  #[idle]
  fn idle(_: idle::Context) -> ! {
    loop {}
  }

  #[task(binds = Interrupt1, priority = 1, shared = [r,l])]
  fn i1(mut cx: i1::Context) {
    cx.shared.r.lock(|r| {
      cx.shared.l.lock(|l| {
        *l = *l + *r;
      });
    });
  }
  #[task(binds = Interrupt2, priority = 2, shared=[l])]
  fn i2(mut cx: i2::Context) {
    cx.shared.l.lock(|l| {
      *l += 2;
    });
  }
  #[task(binds = Interrupt3, priority = 3, shared = [r])]
  fn i3(mut cx: i3::Context) {
    cx.shared.r.lock(|r| {
      *r += 1;
    });
  }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
  loop {}
}
