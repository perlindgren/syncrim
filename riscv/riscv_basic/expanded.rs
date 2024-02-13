#![feature(prelude_import)]
//! examples/lab4_example
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use core::panic::PanicInfo;
use syncrim_clic_rt as _;
use riscv_rt as _;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use clic as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// Holds the maximum priority level for use by async HAL drivers.
    #[no_mangle]
    static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8 = 0u8;
    use clic::Interrupt;
    /// User code end
    ///Shared resources
    struct Shared {}
    ///Local resources
    struct Local {
        pended: bool,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_init_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Core peripherals
        pub core: rtic::export::Peripherals,
        /// Device peripherals (PAC)
        pub device: clic::Peripherals,
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
    }
    impl<'a> __rtic_internal_init_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
            __rtic_internal_init_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                device: clic::Peripherals::steal(),
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
    fn init(_: init::Context) -> (Shared, Local) {
        let pended = false;
        rtic::export::pend(Interrupt::Interrupt2);
        (Shared {}, Local { pended })
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
        rtic::export::run(PRIORITY, || { i1(i1::Context::new()) });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt2() {
        const PRIORITY: u8 = 2u8;
        rtic::export::run(PRIORITY, || { i2(i2::Context::new()) });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt3() {
        const PRIORITY: u8 = 3u8;
        rtic::export::run(PRIORITY, || { i3(i3::Context::new()) });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt4() {
        const PRIORITY: u8 = 4u8;
        rtic::export::run(PRIORITY, || { i4(i4::Context::new()) });
    }
    impl<'a> __rtic_internal_i4LocalResources<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_i4LocalResources {
                pended: &mut *(&mut *__rtic_internal_local_resource_pended.get_mut())
                    .as_mut_ptr(),
                __rtic_internal_marker: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt5() {
        const PRIORITY: u8 = 5u8;
        rtic::export::run(PRIORITY, || { i5(i5::Context::new()) });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt6() {
        const PRIORITY: u8 = 6u8;
        rtic::export::run(PRIORITY, || { i6(i6::Context::new()) });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt7() {
        const PRIORITY: u8 = 7u8;
        rtic::export::run(PRIORITY, || { i7(i7::Context::new()) });
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt8() {
        const PRIORITY: u8 = 8u8;
        rtic::export::run(PRIORITY, || { i8(i8::Context::new()) });
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
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i3_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_i3_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_i3_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i3 {
        #[doc(inline)]
        pub use super::__rtic_internal_i3_Context as Context;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Local resources `i4` has access to
    pub struct __rtic_internal_i4LocalResources<'a> {
        #[allow(missing_docs)]
        pub pended: &'a mut bool,
        #[doc(hidden)]
        pub __rtic_internal_marker: ::core::marker::PhantomData<&'a ()>,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i4_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Local Resources this task has access to
        pub local: i4::LocalResources<'a>,
    }
    impl<'a> __rtic_internal_i4_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_i4_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                local: i4::LocalResources::new(),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i4 {
        #[doc(inline)]
        pub use super::__rtic_internal_i4LocalResources as LocalResources;
        #[doc(inline)]
        pub use super::__rtic_internal_i4_Context as Context;
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i5_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_i5_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_i5_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i5 {
        #[doc(inline)]
        pub use super::__rtic_internal_i5_Context as Context;
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i6_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_i6_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_i6_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i6 {
        #[doc(inline)]
        pub use super::__rtic_internal_i6_Context as Context;
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i7_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_i7_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_i7_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i7 {
        #[doc(inline)]
        pub use super::__rtic_internal_i7_Context as Context;
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i8_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_i8_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_i8_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i8 {
        #[doc(inline)]
        pub use super::__rtic_internal_i8_Context as Context;
    }
    #[allow(non_snake_case)]
    fn i1(_: i1::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    #[allow(non_snake_case)]
    fn i2(_: i2::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        rtic::export::pend(Interrupt::Interrupt1);
        rtic::export::pend(Interrupt::Interrupt4);
    }
    #[allow(non_snake_case)]
    fn i3(_: i3::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    #[allow(non_snake_case)]
    fn i4(cx: i4::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        if !*cx.local.pended {
            *cx.local.pended = true;
            rtic::export::pend(Interrupt::Interrupt6);
        }
    }
    #[allow(non_snake_case)]
    fn i5(_: i5::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    #[allow(non_snake_case)]
    fn i6(_: i6::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        rtic::export::pend(Interrupt::Interrupt8);
        rtic::export::pend(Interrupt::Interrupt3);
    }
    #[allow(non_snake_case)]
    fn i7(_: i7::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
    }
    #[allow(non_snake_case)]
    fn i8(_: i8::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        rtic::export::pend(Interrupt::Interrupt4);
        rtic::export::pend(Interrupt::Interrupt5);
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic0"]
    static __rtic_internal_local_resource_pended: rtic::RacyCell<
        core::mem::MaybeUninit<bool>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
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
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt1,
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
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt2,
            2u8,
        );
        const _: () = if (15usize) <= 3u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt3\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt3,
            3u8,
        );
        const _: () = if (15usize) <= 4u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt4\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt4,
            4u8,
        );
        const _: () = if (15usize) <= 5u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt5\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt5,
            5u8,
        );
        const _: () = if (15usize) <= 6u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt6\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt6,
            6u8,
        );
        const _: () = if (15usize) <= 7u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt7\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt7,
            7u8,
        );
        const _: () = if (15usize) <= 8u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt8\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt8,
            8u8,
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
            __rtic_internal_local_resource_pended
                .get_mut()
                .write(core::mem::MaybeUninit::new(local_resources.pended));
            rtic::export::interrupt::enable();
        });
        idle(idle::Context::new())
    }
}
#[panic_handler]
unsafe fn _panic(_: &PanicInfo) -> ! {
    loop {}
}
