use x86_64::instructions::{nop, port::Port};

pub fn exit(exit_code: u32) -> ! {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code);
    }

    loop {
        nop();
    }
}

pub fn read_cycle() -> u64 {
    unsafe { core::arch::x86_64::_rdtsc() }
}

pub fn debug_write(bytes: &[u8]) {
    unsafe {
        for byte in bytes {
            core::arch::asm!("out 0xe9, al", in("al") *byte);
        }
    }
}
