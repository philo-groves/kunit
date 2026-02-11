#![no_std]
#![cfg_attr(test, no_main)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_harness")]

#[cfg(test)]
extern crate self as kunit;

mod arch;
mod args;
pub mod macros;
mod print;
mod qemu;
pub mod test;

pub use kunit_macros::kunit;
pub use macros::klib::{KlibConfig, KlibConfigBuilder};
pub use test::{runner::runner, split_module_path, split_module_path_len, Test};

/// Maximum length for strings used in this library, to avoid dynamic allocations.
const MAX_STRING_LENGTH: usize = 1024;

/// Initialize the test harness with the given test group. This function should be called
/// before the main test function is called.
///
/// For example, in your lib.rs:
///
/// ```
/// kunit::init_harness("library");
/// test_main();
/// ```
///
/// If you are using the `klib!` macro, this function is called automatically.
///
pub fn init_harness(test_group: &str) {
    args::set_test_group(test_group);
}

/// A panic handler that delegates to the test runner's panic handler. This should be
/// included in libraries which use `kunit` to allow recovery from panics during tests.
///
/// Only include this in test builds.
///
/// For example, in your lib.rs:
///
/// ```
/// #[cfg(test)]
/// #[panic_handler]
/// fn panic(info: &core::panic::PanicInfo) -> ! {
///     kunit::panic(info)
/// }
/// ```
///
/// If you are using the `klib!` macro, this function is included automatically.
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    use crate::test::runner::TestRunner;
    use crate::test::runner::TEST_RUNNER;

    if let Some(runner) = TEST_RUNNER.get() {
        runner.handle_panic(info);
    }

    use heapless::{format, String};

    let test_group = args::get_test_group().unwrap_or("default");
    test::output::write_test_group(test_group, 0);

    let location: String<MAX_STRING_LENGTH> = if let Some(location) = info.location() {
        format!("{}:{}", location.file(), location.line()).unwrap()
    } else {
        String::<MAX_STRING_LENGTH>::try_from("unknown location").unwrap()
    };
    let message = info.message().as_str().unwrap_or("no message");
    let test_name: String<MAX_STRING_LENGTH> = format!("bootstrap::panic").unwrap();

    test::output::write_test_failure(&test_name, location.as_str(), message);
    qemu::exit(qemu::ExitCode::_Failed)
}
