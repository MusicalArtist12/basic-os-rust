#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code)]
#![feature(lazy_cell)]
#![feature(naked_functions)]

pub mod kernel;
use core::{unreachable, panic::PanicInfo};

use crate::kernel::io::{basic_vga_driver::CharAttr, STDOUT, basic_vga_driver::Color};

#[panic_handler]
fn on_panic(panic_info: &PanicInfo) -> ! {
    STDOUT.lock().change_color(CharAttr::new(Color::White, Color::Blue));
    STDOUT.lock().clear_screen();
    
    println!("oh no :(\n");
    println!("{:#?}", panic_info);

    cli!();
    hlt!();
    unreachable!("something is *very* wrong");
}

pub fn main() {
    println!("Welcome to the Main Function!");

    // assert!(2 + 2 == 5);
}