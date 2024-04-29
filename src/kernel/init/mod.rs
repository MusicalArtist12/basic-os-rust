use core::arch::global_asm;

mod multiboot;

global_asm!(include_str!("i386.asm"));
global_asm!(include_str!("x86_64.asm"));