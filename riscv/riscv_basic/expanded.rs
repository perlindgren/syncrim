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
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use hippomenes_core as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// Holds the maximum priority level for use by async HAL drivers.
    #[no_mangle]
    static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8 = 2u8;
    use hippomenes_core::{gpio, interrupt0, timer};
    /// User code end
    ///Shared resources
    struct Shared {
        time_stamp: usize,
        toggled: bool,
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
    fn init(_: init::Context) -> (Shared, Local) {
        timer::Bits::write(0b1111 << 4);
        let toggled = false;
        let time_stamp = 0;
        (Shared { time_stamp, toggled }, Local {})
    }
    impl<'a> __rtic_internal_idleSharedResources<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_idleSharedResources {
                time_stamp: shared_resources::time_stamp_that_needs_to_be_locked::new(
                    priority,
                ),
                __rtic_internal_marker: core::marker::PhantomData,
                priority: priority,
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `idle` has access to
    pub struct __rtic_internal_idleSharedResources<'a> {
        #[allow(missing_docs)]
        pub time_stamp: shared_resources::time_stamp_that_needs_to_be_locked<'a>,
        #[doc(hidden)]
        pub __rtic_internal_marker: core::marker::PhantomData<&'a ()>,
        pub priority: &'a rtic::export::Priority,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_idle_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Shared Resources this task has access to
        pub shared: idle::SharedResources<'a>,
        pub priority: &'a rtic::export::Priority,
    }
    impl<'a> __rtic_internal_idle_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_idle_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                priority,
                shared: idle::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Idle loop
    pub mod idle {
        #[doc(inline)]
        pub use super::__rtic_internal_idleSharedResources as SharedResources;
        #[doc(inline)]
        pub use super::__rtic_internal_idle_Context as Context;
    }
    #[allow(non_snake_case)]
    fn idle(mut cx: idle::Context) -> ! {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        loop {
            let time_stamp = cx.shared.time_stamp.lock(|time_stamp| *time_stamp);
        }
    }
    #[allow(non_snake_case)]
    #[no_mangle]
    unsafe fn Interrupt0() {
        const PRIORITY: u8 = 3u8;
        rtic::export::run(
            PRIORITY,
            || {
                timer_task(
                    timer_task::Context::new(&rtic::export::Priority::new(PRIORITY)),
                )
            },
        );
    }
    impl<'a> __rtic_internal_timer_taskSharedResources<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_timer_taskSharedResources {
                time_stamp: shared_resources::time_stamp_that_needs_to_be_locked::new(
                    priority,
                ),
                toggled: shared_resources::toggled_that_needs_to_be_locked::new(
                    priority,
                ),
                __rtic_internal_marker: core::marker::PhantomData,
                priority: priority,
            }
        }
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    ///Shared resources `timer_task` has access to
    pub struct __rtic_internal_timer_taskSharedResources<'a> {
        #[allow(missing_docs)]
        pub time_stamp: shared_resources::time_stamp_that_needs_to_be_locked<'a>,
        #[allow(missing_docs)]
        pub toggled: shared_resources::toggled_that_needs_to_be_locked<'a>,
        #[doc(hidden)]
        pub __rtic_internal_marker: core::marker::PhantomData<&'a ()>,
        pub priority: &'a rtic::export::Priority,
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_timer_task_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
        /// Shared Resources this task has access to
        pub shared: timer_task::SharedResources<'a>,
        pub priority: &'a rtic::export::Priority,
    }
    impl<'a> __rtic_internal_timer_task_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
            __rtic_internal_timer_task_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
                priority,
                shared: timer_task::SharedResources::new(priority),
            }
        }
    }
    #[allow(non_snake_case)]
    ///Hardware task
    pub mod timer_task {
        #[doc(inline)]
        pub use super::__rtic_internal_timer_taskSharedResources as SharedResources;
        #[doc(inline)]
        pub use super::__rtic_internal_timer_task_Context as Context;
    }
    #[allow(non_snake_case)]
    fn timer_task(mut cx: timer_task::Context) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        cx.shared
            .toggled
            .lock(|toggled| {
                *toggled = !*toggled;
                gpio::Bits::write(*toggled as usize);
            });
        cx.shared
            .time_stamp
            .lock(|time_stamp| {
                *time_stamp = interrupt0::Timestamp::Bits::read();
            });
    }
    #[allow(non_camel_case_types)]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = ".uninit.rtic0"]
    static __rtic_internal_shared_resource_time_stamp: rtic::RacyCell<
        core::mem::MaybeUninit<usize>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::time_stamp_that_needs_to_be_locked<'a> {
        type T = usize;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut usize) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 3u8;
            unsafe {
                rtic::export::lock(
                    __rtic_internal_shared_resource_time_stamp.get_mut() as *mut _,
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
    static __rtic_internal_shared_resource_toggled: rtic::RacyCell<
        core::mem::MaybeUninit<bool>,
    > = rtic::RacyCell::new(core::mem::MaybeUninit::uninit());
    impl<'a> rtic::Mutex for shared_resources::toggled_that_needs_to_be_locked<'a> {
        type T = bool;
        #[inline(always)]
        fn lock<RTIC_INTERNAL_R>(
            &mut self,
            f: impl FnOnce(&mut bool) -> RTIC_INTERNAL_R,
        ) -> RTIC_INTERNAL_R {
            /// Priority ceiling
            const CEILING: u8 = 3u8;
            unsafe {
                rtic::export::lock(
                    __rtic_internal_shared_resource_toggled.get_mut() as *mut _,
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
        pub struct time_stamp_that_needs_to_be_locked<'a> {
            __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
            pub priority: &'a rtic::export::Priority,
        }
        impl<'a> time_stamp_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
                time_stamp_that_needs_to_be_locked {
                    __rtic_internal_p: ::core::marker::PhantomData,
                    priority,
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct toggled_that_needs_to_be_locked<'a> {
            __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
            pub priority: &'a rtic::export::Priority,
        }
        impl<'a> toggled_that_needs_to_be_locked<'a> {
            #[inline(always)]
            pub unsafe fn new(priority: &'a rtic::export::Priority) -> Self {
                toggled_that_needs_to_be_locked {
                    __rtic_internal_p: ::core::marker::PhantomData,
                    priority,
                }
            }
        }
    }
    #[doc(hidden)]
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        rtic::export::assert_send::<usize>();
        rtic::export::assert_send::<bool>();
        rtic::export::interrupt::disable();
        let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
            .into();
        const _: () = if (15usize) <= 3u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt0\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt0,
            3u8,
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
            __rtic_internal_shared_resource_time_stamp
                .get_mut()
                .write(core::mem::MaybeUninit::new(shared_resources.time_stamp));
            __rtic_internal_shared_resource_toggled
                .get_mut()
                .write(core::mem::MaybeUninit::new(shared_resources.toggled));
            rtic::export::interrupt::enable();
        });
        idle(idle::Context::new(&rtic::export::Priority::new(0u8)))
    }
}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
