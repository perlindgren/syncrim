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
/// The RTIC application module
pub mod app {
    /// Always include the device crate which contains the vector table
    use clic as you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml;
    /// Holds the maximum priority level for use by async HAL drivers.
    #[no_mangle]
    static RTIC_ASYNC_MAX_LOGICAL_PRIO: u8 = 1 << 7;
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
        /// Critical section token for init
        pub cs: rtic::export::CriticalSection<'a>,
    }
    impl<'a> __rtic_internal_init_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new(core: rtic::export::Peripherals) -> Self {
            __rtic_internal_init_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
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
        foo::spawn().unwrap();
        (Shared {}, Local {})
    }
    /// Execution context
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    pub struct __rtic_internal_foo_Context<'a> {
        #[doc(hidden)]
        __rtic_internal_p: ::core::marker::PhantomData<&'a ()>,
    }
    impl<'a> __rtic_internal_foo_Context<'a> {
        #[inline(always)]
        #[allow(missing_docs)]
        pub unsafe fn new() -> Self {
            __rtic_internal_foo_Context {
                __rtic_internal_p: ::core::marker::PhantomData,
            }
        }
    }
    /// Spawns the task directly
    #[allow(non_snake_case)]
    #[doc(hidden)]
    pub fn __rtic_internal_foo_spawn() -> Result<(), ()> {
        #[inline(always)]
        fn tait_hack() -> __rtic_internal_foo_F {
            foo(unsafe { foo::Context::new() })
        }
        unsafe {
            if __rtic_internal_foo_EXEC.try_allocate() {
                let f = tait_hack();
                __rtic_internal_foo_EXEC.spawn(f);
                rtic::export::pend(clic::Interrupt::Interrupt0);
                Ok(())
            } else {
                Err(())
            }
        }
    }
    #[allow(non_snake_case)]
    ///Software task
    pub mod foo {
        #[doc(inline)]
        pub use super::__rtic_internal_foo_Context as Context;
        #[doc(inline)]
        pub use super::__rtic_internal_foo_spawn as spawn;
    }
    #[allow(non_snake_case)]
    async fn foo<'a>(_: foo::Context<'a>) {
        use rtic::Mutex as _;
        use rtic::mutex::prelude::*;
        loop {}
    }
    #[allow(non_camel_case_types)]
    type __rtic_internal_foo_F = impl core::future::Future;
    #[allow(non_upper_case_globals)]
    static __rtic_internal_foo_EXEC: rtic::export::executor::AsyncTaskExecutor<
        __rtic_internal_foo_F,
    > = rtic::export::executor::AsyncTaskExecutor::new();
    #[allow(non_snake_case)]
    ///Interrupt handler to dispatch async tasks at priority 2
    #[no_mangle]
    #[export_name = "_interrupt0"]
    unsafe fn Interrupt0() {
        rtic::export::unpend(rtic::export::Interrupt::Interrupt0);
        /// The priority of this interrupt handler
        const PRIORITY: u8 = 2u8;
        rtic::export::run(
            PRIORITY,
            || {
                __rtic_internal_foo_EXEC
                    .poll(|| {
                        __rtic_internal_foo_EXEC.set_pending();
                        rtic::export::pend(clic::Interrupt::Interrupt0);
                    });
            },
        );
    }
    #[doc(hidden)]
    #[no_mangle]
    unsafe extern "C" fn main() -> ! {
        rtic::export::interrupt::disable();
        let mut core: rtic::export::Peripherals = rtic::export::Peripherals::steal()
            .into();
        let _ = you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt0;
        const _: () = if (15usize) <= 2u8 as usize {
            {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "Maximum priority used by interrupt vector \'Interrupt0\' is more than supported by hardware",
                    ),
                );
            };
        };
        rtic::export::enable(
            you_must_enable_the_rt_feature_for_the_pac_in_your_cargo_toml::Interrupt::Interrupt0,
            2u8,
            1u8,
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
        loop {
            continue;
        }
    }
}
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
