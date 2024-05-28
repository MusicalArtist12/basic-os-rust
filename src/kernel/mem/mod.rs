use core::arch::global_asm;

global_asm!(include_str!("bss.s"), options(att_syntax));

pub mod frame_allocator;

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
