#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::{debug_write, disable_interrupts, exit, read_cycle};

#[cfg(target_arch = "aarch64")]
pub use aarch64::{debug_write, disable_interrupts, exit, read_cycle};

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
compile_error!("kunit currently supports only x86_64 and aarch64 targets");
