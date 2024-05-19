#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code)]
#![feature(lazy_cell)]
#![feature(naked_functions)]

pub mod kernel;
use core::panic::PanicInfo;

#[panic_handler]
fn on_panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

pub fn main() {
    println!("Welcome to the Main Function!");
}