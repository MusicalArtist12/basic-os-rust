use crate::{main, sti};
use super::io::basic_vga_driver::{CharAttr, Color};
use super::io::STDOUT;
use super::io::pic;

use crate::println;
use super::interrupts::load_interrupt_handlers;
use super::multiboot::MultibootInfo;

fn successful_boot() {
    STDOUT.lock().clear_screen();
    println!("Boot Successful! Here's a {}", "balloon!");
  
    STDOUT.lock().change_color(CharAttr::new(Color::Red, Color::Black));
    /* 
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
    */

    STDOUT.lock().change_color(CharAttr::new(Color::White, Color::Black));
    
  
}

#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    // pic::initialize(0x20, 0x28);
    pic::mask_all();
    load_interrupt_handlers();
    sti!();

    successful_boot();

    let info = unsafe { MultibootInfo::new(multiboot_information_address) };
    info.log_tags();

    main();

    loop { }

    /* 
    cli!();
    hlt!();
    unreachable!();
    */
}
