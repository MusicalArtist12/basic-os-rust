use core::arch::global_asm;

global_asm!(include_str!("bss.s"), options(att_syntax));

pub mod frame_allocator;
