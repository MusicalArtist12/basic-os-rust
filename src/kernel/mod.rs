pub mod sync;
pub mod io;
pub mod multiboot;
pub mod interrupts;
pub mod boot;
pub mod gdt;
pub mod mem;

use crate::main;

use crate::hlt;
use crate::cli;
use crate::sti;

use io::pic;
use multiboot::MultibootInfo;

#[macro_export]
macro_rules! sti {
    () => {
        unsafe { core::arch::asm!("sti"); }
    };
}

#[macro_export]
macro_rules! cli {
    () => {
        unsafe { core::arch::asm!("cli"); }
    };
}

#[macro_export]
macro_rules! hlt {
    () => {
        unsafe { core::arch::asm!("hlt"); }
        unreachable!("system failed to halt");
    };
}

/* 
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
*/


#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    pic::mask_all(); // pic::initialize(0x20, 0x28);
    interrupts::load_interrupt_handlers();
    sti!();

    let _info = MultibootInfo::new(multiboot_information_address);
    
    main();

    cli!();
    hlt!();
}
