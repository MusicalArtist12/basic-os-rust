use core::arch::asm;

pub fn mask_all() {
    unsafe {
        asm!(r#"
            or $0xff, %al
            outb %al, $0x21

            or $0xff, %al
            outb %al, $0xA1
        "#, options(att_syntax));
    }
}
