#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code)]
#![feature(lazy_cell)]

pub mod kernel;
use core::panic::PanicInfo;
use core::fmt::Write;
use crate::kernel::io::basic_vga_driver::{STDOUT, CharAttr, Color};
#[panic_handler]
fn on_panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

pub fn main(multiboot_addr: usize) {
    STDOUT.lock().default_attr = CharAttr::new(Color::Red, Color::Black);

    STDOUT.lock().clear_screen();

    println!("Hello World, heres a {}", "balloon!");

    println!(r#"
       _..._  ,s$$$s.
    .$$$$$$$s$$ss$$$$,
    $$$sss$$$$s$$$$$$$
    $$ss$$$$$$$$$$$$$$
    '$$$s$$$$$$$$$$$$'
     '$$$$$$$$$$$$$$'
       S$$$$$$$$$$$'
        '$$$$$$$$$'
          '$$$$$'
           '$$$'
             ;
            ;
            ;
            ',
             ;
            ,'
            ;
            ',
             ',
              ;
             '
"#);

    loop {}
}