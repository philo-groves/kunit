use crate::MAX_STRING_LENGTH;
#[cfg(target_arch = "x86_64")]
use conquer_once::spin::OnceCell;
use heapless::String;
#[cfg(target_arch = "x86_64")]
use spin::Mutex;

/// The global serial port instance
#[cfg(target_arch = "x86_64")]
#[allow(dead_code)]
pub static SERIAL1: OnceCell<Mutex<uart_16550::SerialPort>> = OnceCell::uninit();

/// Initialize the global serial port
#[cfg(target_arch = "x86_64")]
#[allow(dead_code)]
fn init_serial() -> Mutex<uart_16550::SerialPort> {
    let mut serial_port = unsafe { uart_16550::SerialPort::new(0x3F8) };
    serial_port.init();
    Mutex::new(serial_port)
}

/// Print to the global serial port
#[doc(hidden)]
pub fn _serial_print(args: core::fmt::Arguments) {
    #[cfg(target_arch = "x86_64")]
    {
        use core::fmt::Write;
        use x86_64::instructions::interrupts;

        interrupts::without_interrupts(|| {
            let serial = SERIAL1.get_or_init(|| init_serial());
            serial
                .lock()
                .write_fmt(args)
                .expect("Printing to serial failed");
        });
    }

    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = args;
    }
}

/// Print to the global serial port
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::print::_serial_print(format_args!($($arg)*));
    };
}

/// Print to the global serial port, with a newline
#[macro_export]
macro_rules! serial_println {
    ($($arg:tt)*) => {
        $crate::print::_serial_print(format_args!($($arg)*));
        $crate::print::_serial_print(format_args!("\n"));
    };
}

/// Print to the debug console (macro helper)
#[doc(hidden)]
pub fn _debugcon_print(args: core::fmt::Arguments) {
    // convert args to string
    use core::fmt::Write;
    let mut s = String::<MAX_STRING_LENGTH>::new();
    s.write_fmt(args).expect("Failed to write to string");

    crate::arch::debug_write(s.as_bytes());
}

/// Print to the debug console
#[macro_export]
macro_rules! debugcon_print {
    ($($arg:tt)*) => {
        $crate::print::_debugcon_print(format_args!($($arg)*));
    };
}

/// Print to the debug console, with a newline
#[macro_export]
macro_rules! debugcon_println {
    ($($arg:tt)*) => {
        $crate::print::_debugcon_print(format_args!($($arg)*));
        $crate::print::_debugcon_print(format_args!("\n"));
    };
}
