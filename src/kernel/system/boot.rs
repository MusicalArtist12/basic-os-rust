use crate::main;
use crate::kernel::io::basic_vga_driver::{STDOUT, CharAttr, Color};
use crate::println;

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
pub extern "C" fn _start(mulitboot_information_address: usize) -> ! {
    successful_boot();

    main();

    loop {}
}
