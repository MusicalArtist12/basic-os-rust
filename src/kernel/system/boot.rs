use crate::main;
use crate::kernel::io::basic_vga_driver::{STDOUT, CharAttr, Color};
use crate::println;
use super::multiboot::multiboot_info::read_multiboot;
use super::gdt::gdt64_code_offset;
use super::interrupts::{init_idt, irq };

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
}




#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    
    // STDOUT.lock().change_color(CharAttr::new(Color::White, Color::Black));
    // read_multiboot(multiboot_information_address as u32);
    
    irq::mask_all();

    init_idt();

    successful_boot();



    main();

    loop { }
}
