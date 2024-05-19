use core::arch::asm;

pub unsafe fn outb(port: u16, command: u8) {
    asm!(
        "outb {x}, %dx", 
        in("dx") port, 
        x = in(reg_byte) command,
        options(att_syntax)
    );
}


pub unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    asm! (
        "inb %dx, {}",
        out(reg_byte) val,
        in("dx") port,
        options(att_syntax)
    );
    return val;
}