use core::arch::global_asm;

mod multiboot_header;
mod boot;
mod interrupt;

global_asm!(include_str!("bootloader.s"),  options(att_syntax));
global_asm!(include_str!("memory.s"),  options(att_syntax));
