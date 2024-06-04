use core::arch::global_asm;

global_asm!(include_str!("bss.s"), options(att_syntax));

pub mod frame_table;
pub mod page_table;

#[macro_export]
macro_rules! megabytes {
    ($num: expr) => {
        $num * 1024 * 1024  
    };
}

#[macro_export]
macro_rules! kilobytes {
    ($num: expr) => {
        $num * 1024 
    };
}

#[macro_export]
macro_rules! gigabytes {
    ($num: expr) => {
        $num * 1024 * 1024 * 1024
    };
}


pub const PAGE_SIZE: usize = kilobytes!(4);
pub const NUM_FRAMES: usize = gigabytes!(4) / PAGE_SIZE;
pub const ENTRY_COUNT: usize = 512;

