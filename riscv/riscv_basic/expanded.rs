#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use core::panic::PanicInfo;
use hippomenes_core::mstatus;
use riscv_rt::entry;
#[allow(non_snake_case)]
#[export_name = "main"]
pub unsafe fn __risc_v_rt__main() -> ! {
  mstatus::Bits::read();
  loop {}
}
#[panic_handler]
fn p(_: &PanicInfo) -> ! {
  loop {}
}
