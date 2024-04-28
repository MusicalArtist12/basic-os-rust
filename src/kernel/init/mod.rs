use core::arch::global_asm;

mod multiboot;

global_asm!(include_str!("entry_point.s"));