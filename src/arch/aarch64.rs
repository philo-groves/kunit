const PSCI_SYSTEM_OFF: u64 = 0x8400_0008;

const PL011_BASE: usize = 0x0900_0000;
const PL011_DR: usize = PL011_BASE + 0x00;
const PL011_FR: usize = PL011_BASE + 0x18;
const PL011_FR_TXFF: u32 = 1 << 5;

pub fn exit(exit_code: u32) -> ! {
    let _ = exit_code;

    unsafe {
        core::arch::asm!(
            "hvc #0",
            in("x0") PSCI_SYSTEM_OFF,
            options(nostack)
        );
    }

    loop {
        unsafe {
            core::arch::asm!("wfi", options(nomem, nostack, preserves_flags));
        }
    }
}

pub fn read_cycle() -> u64 {
    let value: u64;
    unsafe {
        core::arch::asm!("mrs {value}, cntvct_el0", value = out(reg) value, options(nomem, nostack, preserves_flags));
    }
    value
}

pub fn debug_write(bytes: &[u8]) {
    use core::ptr::{read_volatile, write_volatile};

    for &byte in bytes {
        unsafe {
            while read_volatile(PL011_FR as *const u32) & PL011_FR_TXFF != 0 {
                core::arch::asm!("nop", options(nomem, nostack, preserves_flags));
            }
            write_volatile(PL011_DR as *mut u32, byte as u32);
        }
    }
}
