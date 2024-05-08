use core::arch::global_asm;

mod gdt;
mod interrupts;
mod boot;
mod multiboot;

global_asm!(include_str!("init.s"), options(att_syntax));
global_asm!(include_str!("bss.s"), options(att_syntax));

