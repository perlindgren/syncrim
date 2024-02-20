#![feature(prelude_import)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use core::panic::PanicInfo;
use riscv_rt as _;
use syncrim_clic_rt as _;
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use syncrim_pac as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// Holds the maximum priority level for use by async HAL drivers.
    #[no_mangle]
    static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8 = 1u8;
    /// User code end
    ///Shared resources
    struct Shared {
        low_prio_r: u32,
        high_prio_r: u32,
    }
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
        pub device: syncrim_pac::Peripherals,
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
    }
    impl<'a> __rtic_internal_init_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
            __rtic_internal_init_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                device: syncrim_pac::Peripherals::steal(),
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
    fn init(_cx: init::Context) -> (Shared, Local) {
        let low_prio_r = 0;
        let high_prio_r = 0;
        rtic::export::pend(clic::Interrupt2);
        (Shared { low_prio_r, high_prio_r }, Local {})
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
    unsafe fn Interrupt2() {
        const PRIORITY: u8 = 2u8;
        rtic::export::run(
            PRIORITY,
            || { i2(i2::Context::new(&rtic::export::Priority::new(PRIORITY))) },
        );
    }
    impl<'a> __rtic_internal_i2SharedResources<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_i2SharedResources {
                low_prio_r: shared_resources::low_prio_r_that_needs_to_be_locked::new(
                    priority,
                ),
                high_prio_r: shared_resources::high_prio_r_that_needs_to_be_locked::new(
                    priority,
                ),
                __rtic_internal_marker: core::marker::PhantomData,
                priority: priority,
            }
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt3() {
        const PRIORITY: u8 = 3u8;
        rtic::export::run(
            PRIORITY,
            || { i3(i3::Context::new(&rtic::export::Priority::new(PRIORITY))) },
        );
    }
    impl<'a> __rtic_internal_i3SharedResources<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_i3SharedResources {
                low_prio_r: shared_resources::low_prio_r_that_needs_to_be_locked::new(
                    priority,
                ),
                __rtic_internal_marker: core::marker::PhantomData,
                priority: priority,
            }
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt4() {
        const PRIORITY: u8 = 4u8;
        rtic::export::run(
            PRIORITY,
            || { i4(i4::Context::new(&rtic::export::Priority::new(PRIORITY))) },
        );
    }
    impl<'a> __rtic_internal_i4SharedResources<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_i4SharedResources {
                high_prio_r: shared_resources::high_prio_r_that_needs_to_be_locked::new(
                    priority,
                ),
                __rtic_internal_marker: core::marker::PhantomData,
                priority: priority,
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `i2` has access to
    pub struct __rtic_internal_i2SharedResources<'a> {
        #[allow(missing_docs)]
        pub low_prio_r: shared_resources::low_prio_r_that_needs_to_be_locked<'a>,
        #[allow(missing_docs)]
        pub high_prio_r: shared_resources::high_prio_r_that_needs_to_be_locked<'a>,
        #[doc(hidden)]
        pub __rtic_internal_marker: core::marker::PhantomData<&'a ()>,
        pub priority: &'a rtic::export::Priority,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i2_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Shared Resources this task has access to
        pub shared: i2::SharedResources<'a>,
        pub priority: &'a rtic::export::Priority,
    }
    impl<'a> __rtic_internal_i2_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_i2_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                priority,
                shared: i2::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i2 {
        #[doc(inline)]
        pub use super::__rtic_internal_i2SharedResources as SharedResources;
        #[doc(inline)]
        pub use super::__rtic_internal_i2_Context as Context;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `i3` has access to
    pub struct __rtic_internal_i3SharedResources<'a> {
        #[allow(missing_docs)]
        pub low_prio_r: shared_resources::low_prio_r_that_needs_to_be_locked<'a>,
        #[doc(hidden)]
        pub __rtic_internal_marker: core::marker::PhantomData<&'a ()>,
        pub priority: &'a rtic::export::Priority,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i3_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Shared Resources this task has access to
        pub shared: i3::SharedResources<'a>,
        pub priority: &'a rtic::export::Priority,
    }
    impl<'a> __rtic_internal_i3_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_i3_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                priority,
                shared: i3::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i3 {
        #[doc(inline)]
        pub use super::__rtic_internal_i3SharedResources as SharedResources;
        #[doc(inline)]
        pub use super::__rtic_internal_i3_Context as Context;
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `i4` has access to
    pub struct __rtic_internal_i4SharedResources<'a> {
        #[allow(missing_docs)]
        pub high_prio_r: shared_resources::high_prio_r_that_needs_to_be_locked<'a>,
        #[doc(hidden)]
        pub __rtic_internal_marker: core::marker::PhantomData<&'a ()>,
        pub priority: &'a rtic::export::Priority,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_i4_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Shared Resources this task has access to
        pub shared: i4::SharedResources<'a>,
        pub priority: &'a rtic::export::Priority,
    }
    impl<'a> __rtic_internal_i4_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_i4_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                priority,
                shared: i4::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod i4 {
        #[doc(inline)]
        pub use super::__rtic_internal_i4SharedResources as SharedResources;
        #[doc(inline)]
        pub use super::__rtic_internal_i4_Context as Context;
    }
    #[allow(non_snake_case)]
    fn i2(mut cx: i2::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        cx.shared
            .high_prio_r
            .lock(|high_prio_r| {
                cx.shared
                    .low_prio_r
                    .lock(|low_prio_r| {
                        rtic::export::pend(clic::Interrupt4);
                        *low_prio_r += 1;
                        *high_prio_r += 1;
                    });
                *high_prio_r += 1;
            });
        cx.shared
            .low_prio_r
            .lock(|low_prio_r| {
                rtic::export::pend(clic::Interrupt3);
                rtic::export::pend(clic::Interrupt4);
                *low_prio_r += 1;
            })
    }
    #[allow(non_snake_case)]
    fn i3(mut cx: i3::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        cx.shared
            .low_prio_r
            .lock(|low_prio_r| {
                *low_prio_r += 1;
            });
    }
    #[allow(non_snake_case)]
    fn i4(mut cx: i4::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        cx.shared
            .high_prio_r
            .lock(|high_prio_r| {
                *high_prio_r += 1;
            });
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic0"]
    static __rtic_internal_shared_resource_low_prio_r: rtic::RacyCell<
        core::mem::MaybeUninit<u32>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::low_prio_r_that_needs_to_be_locked<'a> {
        type T = u32;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut u32) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 3u8;
            unsafe {
                rtic::export::lock(
                    __rtic_internal_shared_resource_low_prio_r.get_mut() as *mut _,
                    self.priority,
                    CEILING,
                    f,
                )
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic1"]
    static __rtic_internal_shared_resource_high_prio_r: rtic::RacyCell<
        core::mem::MaybeUninit<u32>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::high_prio_r_that_needs_to_be_locked<'a> {
        type T = u32;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut u32) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 4u8;
            unsafe {
                rtic::export::lock(
                    __rtic_internal_shared_resource_high_prio_r.get_mut() as *mut _,
                    self.priority,
                    CEILING,
                    f,
                )
            }
        }
    }
    mod shared_resources {
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct low_prio_r_that_needs_to_be_locked<'a> {
            __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
            pub priority: &'a rtic::export::Priority,
        }
        impl<'a> low_prio_r_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
                low_prio_r_that_needs_to_be_locked {
                    __rtic_internal_p: ::core::marker::PhantomData,
                    priority,
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct high_prio_r_that_needs_to_be_locked<'a> {
            __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
            pub priority: &'a rtic::export::Priority,
        }
        impl<'a> high_prio_r_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
                high_prio_r_that_needs_to_be_locked {
                    __rtic_internal_p: ::core::marker::PhantomData,
                    priority,
                }
            }
        }
    }
    #[doc(hidden)]
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        rtic::export::assert_send::<u32>();
        rtic::export::interrupt::disable();
        let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
            .into();
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
            __rtic_internal_shared_resource_low_prio_r
                .get_mut()
                .write(core::mem::MaybeUninit::new(shared_resources.low_prio_r));
            __rtic_internal_shared_resource_high_prio_r
                .get_mut()
                .write(core::mem::MaybeUninit::new(shared_resources.high_prio_r));
            rtic::export::interrupt::enable();
        });
        idle(idle::Context::new())
    }
}
#[panic_handler]
unsafe fn _panic(_: &PanicInfo) -> ! {
    loop {}
}
