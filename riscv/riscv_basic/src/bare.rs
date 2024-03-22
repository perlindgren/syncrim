#![no_std]
#![no_main]
use core::panic::PanicInfo;
use hippomenes_core::{gpio, i0_vec, interrupt0, mstatus::MIE, timer};
use hippomenes_rt::entry;

static mut TOGGLED: bool = false;
static mut TIMESTAMP: usize = 0;
#[entry]
unsafe fn main() -> ! {
  unsafe {
    MIE::set();
  } // enable global interrupts
  unsafe {
    i0_vec::Bits::write(
      Interrupt0 as usize >> 2, // set up Interrupt0 address
    );
  }
  unsafe {
    timer::Bits::write(0b1111 << 4);
  } // trigger timer interrupts every 16th cycle

  unsafe {
    interrupt0::Priority::set(3);
  } // set priority of timer interrupt to 3
  unsafe {
    interrupt0::Enabled::set();
  } // enable timer interrupt
  loop {} // wait for interrupt
}

#[no_mangle]
fn Interrupt0() {
  unsafe { TOGGLED = !TOGGLED }; // flip the global TOGGLED flag
  unsafe {
    gpio::Bits::write(
      TOGGLED as usize, // write the value of TOGGLED to GPIO pin 0
    );
  }
  unsafe {
    // write the interrupt0 time stamp into the global TIMESTAMP variable
    TIMESTAMP = interrupt0::Timestamp::Bits::read();
  }
}

#[panic_handler]
fn p(_: &PanicInfo) -> ! {
  loop {}
}
