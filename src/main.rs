#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

pub mod kernel;

use core::panic::PanicInfo;

#[panic_handler]
fn on_panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}