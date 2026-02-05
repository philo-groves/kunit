/// `klib!` function-like macro
#[macro_export]
macro_rules! klib {
    // test group only
    ($test_group:literal) => {
        $crate::klib!($test_group, klib_config = &ktest::KlibConfig::new_default());
    };

    // test group + klib config
    ($test_group:literal, klib_config = &$klib_config:expr) => {
        #[cfg(test)] // it is important to only include this code in test builds
        const _: () = {
            // note: the triple underscore (___) prefix is to avoid name collisions

            #[used]
            #[unsafe(link_section = ".requests_start_marker")]
            static ___START_MARKER: limine::request::RequestsStartMarker = limine::request::RequestsStartMarker::new();

            #[used]
            #[unsafe(link_section = ".requests_end_marker")]
            static ___END_MARKER: limine::request::RequestsEndMarker = limine::request::RequestsEndMarker::new();

            static ___KLIB_CONFIG: ktest::KlibConfig = $klib_config;

            #[panic_handler]
            fn ___panic(info: &core::panic::PanicInfo) -> ! {
                ktest::panic(info)
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn _start() -> ! {
                ktest::init_harness($test_group);

                if let Some(before_tests) = ___KLIB_CONFIG.before_tests {
                    before_tests();
                }

                test_main();

                if let Some(after_tests) = ___KLIB_CONFIG.after_tests {
                    after_tests();
                }

                loop {
                    // It may seem preferable to use the `x86_64` crate's `hlt` instruction here, 
                    // but that would require any crates using this macro to depend on `x86_64`,
                    // which is not desirable. Using inline assembly avoids that dependency.
                    //
                    // note: this is the exact same instruction as `x86_64::instructions::hlt()`
                    #[cfg(target_arch = "x86_64")]
                    unsafe { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)); }

                    // hlt is not available on aarch64, so we use wfi instead
                    #[cfg(target_arch = "aarch64")]
                    unsafe { core::arch::asm!("wfi", options(nomem, nostack, preserves_flags)); }
                }
            }
        };
    };
}

pub struct KlibConfig {
    pub before_tests: Option<fn()>,
    pub after_tests: Option<fn()>
}

impl KlibConfig {
    pub const fn new_default() -> Self {
        KlibConfig {
            before_tests: None,
            after_tests: None
        }
    }
}

pub struct KlibConfigBuilder {
    pub before_tests: Option<fn()>,
    pub after_tests: Option<fn()>
}

impl KlibConfigBuilder {
    pub const fn new_default() -> Self {
        KlibConfigBuilder {
            before_tests: None,
            after_tests: None
        }
    }

    pub const fn new(before_tests: Option<fn()>, after_tests: Option<fn()>) -> Self {
        KlibConfigBuilder {
            before_tests,
            after_tests
        }
    }

    pub const fn build(self) -> KlibConfig {
        KlibConfig {
            before_tests: self.before_tests,
            after_tests: self.after_tests
        }
    }

    pub const fn before_tests(mut self, before_tests: fn()) -> Self {
        self.before_tests = Some(before_tests);
        self
    }

    pub const fn after_tests(mut self, after_tests: fn()) -> Self {
        self.after_tests = Some(after_tests);
        self
    }
}
