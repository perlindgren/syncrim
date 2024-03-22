#![no_std]
#![no_main]
use core::panic::PanicInfo;
use hippomenes_rt as _;

#[rtic::app(device = hippomenes_core)]
mod app {
  use hippomenes_core::{I0Timestamp, Pin, Pin0};
  use hippomenes_hal::UART;
  #[shared]
  struct Shared {
    dummy: bool,
  }

  #[local]
  struct Local {}

  #[init]
  fn init(cx: init::Context) -> (Shared, Local) {
    let peripherals = cx.device;
    let pin0 = peripherals.gpio.pins().pin0;
    let timer = peripherals.timer;
    let mut uart = UART::new(pin0, timer, 10_000);
    let buffer = [48, 49, 50, 51, 52, 53, 54, 55];
    uart.send(buffer);
    (Shared { dummy: true }, Local {})
  }

  #[idle]
  fn idle(_: idle::Context) -> ! {
    loop {}
  }
}

#[panic_handler]
fn p(_: &PanicInfo) -> ! {
  loop {}
}
