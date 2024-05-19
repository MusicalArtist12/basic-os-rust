pub mod basic_vga_driver;
pub mod pic;

use super::sync::mutex::Mutex;
use basic_vga_driver::{Terminal, Color, CharAttr};
use core::fmt::{self, Write};

pub static STDOUT: Mutex<Terminal> = Mutex::new(
    Terminal::vga_text_mode(CharAttr::new(Color::White, Color::Black))
);

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate:::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    
    STDOUT.lock().write_fmt(args).unwrap();
}