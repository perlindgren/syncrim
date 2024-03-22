#![no_std]
#![no_main]
use core::panic::PanicInfo;
use hippomenes_rt as _;

#[rtic::app(device = hippomenes_core)]
mod app {
  use hippomenes_core::{I0Timestamp, Pin, Pin0};
  #[shared]
  struct Shared {
    dummy: bool,
  }

  #[local]
  struct Local {
    led: Pin0,
    timestamp_reg: I0Timestamp,
  }

  #[init]
  fn init(cx: init::Context) -> (Shared, Local) {
    let peripherals = cx.device;
    let pin0 = peripherals.gpio.pins().pin0;
    let timer = peripherals.timer;
    let timestamp_reg = peripherals.i0_timestamp;
    timer.counter_top().write(15);
    (
      Shared { dummy: true },
      Local {
        led: pin0,
        timestamp_reg,
      },
    )
  }

  #[idle]
  fn idle(_: idle::Context) -> ! {
    loop {}
  }

  #[task(
      binds = Interrupt0,
      priority = 3,
      local = [
        time_stamp: usize = 0,
        toggled: bool = false,
        led,
        timestamp_reg
      ],shared = [dummy]
  )]
  fn timer_task(cx: timer_task::Context) {
    *cx.local.toggled = !*cx.local.toggled;
    if *cx.local.toggled {
      cx.local.led.set_low();
    } else {
      cx.local.led.set_high();
    }
    //cx.local..write(*cx.local.toggled as usize);
    *cx.local.time_stamp = cx.local.timestamp_reg.read();
  }
}

#[panic_handler]
fn p(_: &PanicInfo) -> ! {
  loop {}
}
