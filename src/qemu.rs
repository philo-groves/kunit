/// Exit QEMU with the given exit code. This function will not return.
pub fn exit(exit_code: ExitCode) -> ! {
    crate::arch::exit(exit_code as u32)
}

/// Exit codes for QEMU. These codes are written to the I/O port `0xf4`
/// to signal QEMU to exit with the given code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    _Failed = 0x11,
}
