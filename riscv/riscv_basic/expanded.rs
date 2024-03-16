#![feature(prelude_import)]
#![no_std]
#![no_main]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use core::panic::PanicInfo;
use hippomenes_rt as _;
use hippomenes_rt::entry;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use hippomenes_core as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// Holds the maximum priority level for use by async HAL drivers.
    #[no_mangle]
    static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8 = 0u8;
    use hippomenes_core::{interrupt1, interrupt2};
    /// User code end
    ///Shared resources
    struct Shared {}
    ///Local resources
    struct Local {}
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_init_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Core peripherals
        pub core: rtic::export::Peripherals,
        /// Device peripherals (PAC)
        pub device: hippomenes_core::Peripherals,
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
    }
    impl<'a> __rtic_internal_init_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
            __rtic_internal_init_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                device: hippomenes_core::Peripherals::steal(),
                cs: rtic::export::CriticalSection::new(),
                core,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Initialization function
    pub mod init {
        #[doc(inline)]
        pub use super::__rtic_internal_init_Context as Context;
    }
    #[inline(always)]
    #[allow(non_snake_case)]
    fn init(cx: init::Context) -> (Shared, Local) {
        rtic::export::pend(interrupt1::Interrupt1);
        rtic::export::pend(interrupt2::Interrupt2);
        (Shared {}, Local {})
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_idle_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_idle_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_idle_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Idle loop
    pub mod idle {
        #[doc(inline)]
        pub use super::__rtic_internal_idle_Context as Context;
    }
    #[allow(non_snake_case)]
    fn idle(_: idle::Context) -> ! {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        loop {}
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt1() {
        const PRIORITY: u8 = 1u8;
        rtic::export::run(
            PRIORITY,
            || { i1(i1::Context::new(&rtic::export::Priority::new(PRIORITY))) },
        );
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt2() {
        const PRIORITY: u8 = 2u8;
        rtic::export::run(
            PRIORITY,
            || { i2(i2::Context::new(&rtic::export::Priority::new(PRIORITY))) },
        );
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i1_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_i1_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_i1_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i1 {
        #[doc(inline)]
        pub use super::__rtic_internal_i1_Context as Context;
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i2_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_i2_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_i2_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i2 {
        #[doc(inline)]
        pub use super::__rtic_internal_i2_Context as Context;
    }
    #[allow(non_snake_case)]
    fn i1(mut cx: i1::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    #[allow(non_snake_case)]
    fn i2(mut cx: i2::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    #[doc(hidden)]
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        rtic::export::interrupt::disable();
        let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
            .into();
        const _: () = if (15usize) <= 1u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt1\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt1,
            1u8,
        );
        const _: () = if (15usize) <= 2u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt2\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt2,
            2u8,
        );
        #[inline(never)]
        fn __rtic_init_resources<F>(f: F)
        where
            F: FnOnce(),
        {
            f();
        }
        __rtic_init_resources(|| {
            let (shared_resources, local_resources) = init(
                init::Context::new(core.into()),
            );
            rtic::export::interrupt::enable();
        });
        idle(idle::Context::new())
    }
}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
