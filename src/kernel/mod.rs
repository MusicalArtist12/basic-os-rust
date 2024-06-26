pub mod sync;
pub mod io;
pub mod multiboot;
pub mod interrupts;
pub mod boot;
pub mod gdt;
pub mod mem;

use crate::main;
use crate::kernel::mem::frame_table::FrameTable;

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



#[no_mangle]
pub extern "C" fn _start(multiboot_information_address: usize) -> ! {
    pic::mask_all(); // pic::initialize(0x20, 0x28);
    interrupts::load_interrupt_handlers();
    sti!();

    let info = MultibootInfo::new(multiboot_information_address);

    let memmap = info.memmap().expect("some");
    let elfsymbols = info.elfsymbols().expect("some");

    let free_area = memmap.get_entries();
    let kernel_start = elfsymbols.get_section_headers().min().expect("kernel_start").addr() as usize;
    let kernel_end = elfsymbols.get_section_headers().max().expect("kernel_end").addr() as usize;
    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + info.header.size as usize;
    
    let mut allocator = FrameTable::new(
        multiboot_start,
        multiboot_end,
        kernel_start,
        kernel_end,
        free_area
    );

    let mem = allocator.total_memory();
    let used = allocator.used_space();

    /*
     
    // let slow_used = allocator.slow_used_space();
    println!("{} MB", mem as f32 / (1024 * 1024) as f32);
    println!("{} MB used", used as f32 / (1024 * 1024) as f32);
    // println!("{} MB used - slow validation", slow_used as f32 / (1024 * 1024) as f32);
    println!("{} MB available", (mem - used) as f32 / (1024 * 1024) as f32);

    

    */

    main();
    cli!();
    hlt!();
}
