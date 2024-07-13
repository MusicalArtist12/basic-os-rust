use core::arch::global_asm;

global_asm!(include_str!("bss.s"), options(att_syntax));

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize; 
/*
    bytes 0  - 11 = P1
    bytes 12 - 20 = P2
    bytes 21 - 29 = P3
    bytes 30 - 39 = P4
*/
