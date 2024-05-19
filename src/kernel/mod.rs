pub mod io;
pub mod sync;
mod interrupts;
mod multiboot;
mod boot;
mod gdt;

use core::arch::global_asm;

global_asm!(include_str!("init.s"), options(att_syntax));
global_asm!(include_str!("bss.s"), options(att_syntax));

