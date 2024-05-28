pub mod sync;
pub mod io;
pub mod multiboot;
pub mod interrupts;
pub mod boot;
pub mod gdt;
pub mod mem;

use crate::kernel::mem::frame_allocator::FrameAllocator;
use crate::main;

use crate::hlt;
use crate::cli;
use crate::sti;

use io::pic;
use multiboot::*;
use crate::println;

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

    let info = MultibootInfo::new(multiboot_information_address);

    let memmap = info.memmap().expect("some");
    let elfsymbols = info.elfsymbols().expect("some");

    /* 
    for i in memmap.get_entries() {
        println!("{:x?}", i);
    }
    */

    let free_area = memmap.get_entries();
    let kernel_start = elfsymbols.get_section_headers().min().expect("kernel_start").addr() as usize;
    let kernel_end = elfsymbols.get_section_headers().max().expect("kernel_end").addr() as usize;
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + info.header.size as usize;
    
    
    let allocator = FrameAllocator::new(
        multiboot_start,
        multiboot_end,
        kernel_start,
        kernel_end,
        free_area
    );

    let mem = allocator.total_memory();
    let used = allocator.used_space();

    println!("{} KB", mem as f32 / (1024) as f32);
    println!("{} KB used", used as f32 / (1024) as f32);
    println!("{} KB available", (mem - used) as f32 / 1024 as f32);

    main();

    cli!();
    hlt!();
}
