use core::arch::{global_asm, asm};

global_asm!(include_str!("gdt.s"), options(att_syntax));

pub fn gdt64_code_offset() -> u16 {
    let ebx: u32;
    unsafe {
        asm!(r#"
            movl $gdt64_code_offset, {:e}
        "#,options(att_syntax), out(reg) ebx);
    };
    ebx as u16
}

pub fn gdt64_data_offset() -> u16 {
    let ebx: u32;
    unsafe {
        asm!(r#"
            movl $gdt64_data_offset, {:e}
        "#,options(att_syntax), out(reg) ebx);
    };
    ebx as u16
}

pub fn gdt64_pointer() -> u16 {
    let ebx: u32;
    unsafe {
        asm!(r#"
            movl $gdt64_pointer, {:e}
        "#,options(att_syntax), out(reg) ebx);
    };
    ebx as u16
}
