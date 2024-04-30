use core::arch::global_asm;
mod multiboot;


global_asm!(include_str!("bootloader.asm"));

global_asm!(r#"
.extern gdt_code_seg
.extern long_mode_init
.code32
.section .init.text
long_jump:
    ljmp  $gdt64_code_seg, $long_mode_start
"#, options(att_syntax));
