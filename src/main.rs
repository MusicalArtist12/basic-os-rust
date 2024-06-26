#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(dead_code)]
#![feature(naked_functions)]

pub mod kernel;
use core::{unreachable, panic::PanicInfo};

use crate::kernel::io::{basic_vga_driver::CharAttr, STDOUT, basic_vga_driver::Color};

#[panic_handler]
fn on_panic(panic_info: &PanicInfo) -> ! {
    cli!();
    unsafe { STDOUT.force_unlock() };
    STDOUT.lock().change_color(CharAttr::new(Color::White, Color::Blue));
    // STDOUT.lock().clear_screen();
    
    println!("oh no :(\n");
    println!("{:#?}", panic_info);

    hlt!();
}

pub fn main() {
    println!("Welcome to the Main Function! Here's a Balloon");

    STDOUT.lock().change_color(CharAttr::new(Color::Red, Color::Black));
    
    println!(r#"
           _..._  ,s$$$$s.
        .$$$$$$$s$$$$ss$$$$,
        $$$sss$$$$$$s$$$$$$$
        $$ss$$$$$$$$$$$$$$$$
        '$$$s$$$$$$$$$$$$$$'
         '$$$$$$$$$$$$$$$$'
           '$$$$$$$$$$$$$'
            '$$$$$$$$$$$'
              '$$$$$$$'
               '$$$$$'
                 "$" 
                  ;
                 ;
                 ;            
                  ;
                 ,'
                 ;
                 ',
                  ',
                   ;
                  '
    "#);
    

    STDOUT.lock().change_color(CharAttr::new(Color::White, Color::Black));
}