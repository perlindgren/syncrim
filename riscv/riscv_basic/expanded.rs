#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use core::{arch::asm, panic::PanicInfo, ops, marker::PhantomData};
use riscv::register::{mtvec, mstatus};
use critical_section;
use riscv_rt::entry;
use volatile_register::RW;
mod mintthresh {
    use crate::{write_csr_as_usize, read_csr_as_usize, write_csr, read_csr};
    /// Writes the CSR
    #[inline]
    #[allow(unused_variables)]
    unsafe fn _write(bits: usize) {
        match () {
            #[cfg(not(riscv))]
            () => ::core::panicking::panic("not implemented"),
        }
    }
    /// Writes the CSR
    #[inline]
    pub fn write(bits: usize) {
        unsafe { _write(bits) }
    }
    /// Reads the CSR
    #[inline]
    unsafe fn _read() -> usize {
        match () {
            #[cfg(not(riscv))]
            () => ::core::panicking::panic("not implemented"),
        }
    }
    /// Reads the CSR
    #[inline]
    pub fn read() -> usize {
        unsafe { _read() }
    }
}
#[allow(non_snake_case)]
#[export_name = "main"]
pub unsafe fn __risc_v_rt__main() -> ! {
    mintthresh::write(0);
    mstatus::set_mie();
    let mut clic = Peripherals::steal().CLIC;
    CLIC::unmask(Interrupts::Interrupt0);
    clic.set_priority(Interrupts::Interrupt0, 2);
    CLIC::pend(Interrupts::Interrupt0);
    loop {}
}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
#[no_mangle]
pub unsafe fn _setup_interrupts() {
    mtvec::write(&_VECTOR_TABLE.handler0 as *const _ as usize, mtvec::TrapMode::Vectored)
}
#[repr(C)]
struct VectorTable {
    pub handler0: unsafe extern "C" fn(),
    pub handler1: unsafe extern "C" fn(),
    pub handler2: unsafe extern "C" fn(),
}
extern "C" {
    fn handler_2();
}
static CLIC_WIDTH: usize = 32;
#[link_section = ".data"]
static _VECTOR_TABLE: VectorTable = VectorTable {
    handler0: handler_0,
    handler1: handler_1,
    handler2: handler_2,
};
#[no_mangle]
unsafe extern "C" fn handler_0() {
    CLIC::unpend(Interrupts::Interrupt0);
    asm!("mret")
}
#[no_mangle]
unsafe extern "C" fn handler_1() {
    asm!(
        "\n        li \ta0, 0x1000\n        sb \tzero, 4(a0) \t//unpend self\n        mret\n    "
    )
}
#[repr(u16)]
pub enum Interrupts {
    Interrupt0 = 0,
    Interrupt1 = 1,
    Interrupt2 = 2,
}
#[automatically_derived]
impl ::core::clone::Clone for Interrupts {
    #[inline]
    fn clone(&self) -> Interrupts {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Interrupts {}
unsafe impl InterruptNumber for Interrupts {
    fn number(&self) -> u16 {
        *self as u16
    }
}
#[repr(C)]
pub struct RegisterBlock {
    pub interrupts: [[RW<u8>; 4]; 4096],
}
pub unsafe trait InterruptNumber {
    fn number(&self) -> u16;
}
pub struct Peripherals {
    CLIC: CLIC,
}
static mut TAKEN: bool = false;
impl Peripherals {
    /// Returns all the core peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { TAKEN } { None } else { Some(unsafe { Peripherals::steal() }) }
        })
    }
    /// Unchecked version of `Peripherals::take`
    #[inline]
    pub unsafe fn steal() -> Self {
        TAKEN = true;
        Peripherals {
            CLIC: CLIC { _marker: PhantomData },
        }
    }
}
/// Nested Vector Interrupt Controller
#[allow(clippy::upper_case_acronyms)]
pub struct CLIC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLIC {}
impl CLIC {
    /// Pointer to the register block
    pub const PTR: *const RegisterBlock = 0x0000_1000 as *const _;
}
impl ops::Deref for CLIC {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl CLIC {
    /// Disables `interrupt`
    #[inline]
    pub fn mask<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][1].write(0) }
    }
    /// Enables interrupt
    #[inline]
    pub unsafe fn unmask<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][1].write(1) }
    }
    /// Returns the CLIC priority of interrupt
    #[inline]
    pub fn get_priority<I>(interrupt: I) -> u8
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][3].read() }
    }
    /// Checks if interrupt is enabled
    #[inline]
    pub fn is_enabled<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][1].read() != 0 }
    }
    /// Checks if interrupt is pending
    #[inline]
    pub fn is_pending<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][0].read() != 0 }
    }
    /// Forces interrupt into pending state
    #[inline]
    pub fn pend<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][0].write(1) }
    }
    /// Sets the priority of interrupt to prio
    #[inline]
    pub unsafe fn set_priority<I>(&mut self, interrupt: I, prio: u8)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][3].write(prio) }
    }
    /// Clears interrupt's pending state
    #[inline]
    pub fn unpend<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][0].write(0) }
    }
}
