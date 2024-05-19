use crate::main;
use super::io::basic_vga_driver::{CharAttr, Color};
use super::io::STDOUT;
use super::io::pic;

use crate::println;
use super::interrupts::set_interrupt_handlers;
use super::multiboot::multiboot_info::read_multiboot;

fn successful_boot() {
    STDOUT.lock().clear_screen();
    println!("Boot Successful! Here's a {}", "balloon!");
  
    STDOUT.lock().change_color(CharAttr::new(Color::Red, Color::Black));
    
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


    STDOUT.lock().change_color(CharAttr::new(Color::White, Color::Black));
    
  
}

#[no_mangle]
pub extern "C" fn _start(_multiboot_information_address: usize) -> ! {
    pic::initialize(0x20, 0x28);

    set_interrupt_handlers();

    successful_boot();

    // read_multiboot(multiboot_information_address as u32);
    
    main();

    loop { }
}
