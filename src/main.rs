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

    interface.print_string("hello world!\n", None);
    interface.print_string("nice to meet you!\n", None);

    loop {}
}