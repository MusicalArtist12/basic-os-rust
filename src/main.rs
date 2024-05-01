#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code)]

pub mod kernel;

use core::panic::PanicInfo;

use kernel::io::basic_vga_driver::{CharAttr, Terminal, Color};

#[panic_handler]
fn on_panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

pub fn main() {
    let mut interface = Terminal::vga_text_mode(CharAttr::new(Color::Black, Color::White));
    
    interface.clear_screen();

    for _ in 0..5 {
        interface.print_char(b'H', None);
        interface.print_char(b'E', None);
        interface.print_char(b'L', None);
        interface.print_char(b'L', None);
        interface.print_char(b'O', None);
        interface.print_char(b' ', None);
        interface.print_char(b'W', None);
        interface.print_char(b'O', None);
        interface.print_char(b'R', None);
        interface.print_char(b'L', None);
        interface.print_char(b'D', None);
        interface.print_char(b'\n', None);
    }


    loop {}
}