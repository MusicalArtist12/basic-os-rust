use core::arch::asm;
use crate::kernel::interrupts::{IDT, ExceptionStackFrame};
use crate::{isr_wrapper, println};
use super::io_port::{inb, outb};

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


const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const PIC_EOI: u8 = 0x20;

const KEYBOARD_LINE: u8 = 1;


pub fn initialize(offset1: u8, offset2: u8) {
    assert!(offset1 + 8 <= offset2);

    unsafe {
        let a1 = inb(PIC1_DATA);
        let a2 = inb(PIC2_DATA);

        outb(PIC1_COMMAND, 0x10 | 0x01);
        outb(PIC2_COMMAND, 0x10 | 0x01);

        outb(PIC1_DATA, offset1);
        outb(PIC1_DATA, offset2);

        // master-slave relationship
        outb(PIC1_DATA, 4);
        outb(PIC1_DATA, 2);

        // 8086 mode
        outb(PIC1_DATA, 0x01);
        outb(PIC1_DATA, 0x01);

        outb(PIC1_DATA, a1);
        outb(PIC1_DATA, a2);

        for i in offset1..(offset1 + 8) {
            IDT.lock().set_handler(i, isr_wrapper!(pic1_interrupt_handler));
        }

        for i in offset2..(offset2 + 8) {
            IDT.lock().set_handler(i, isr_wrapper!(pic2_interrupt_handler));
        }
        
        outb(PIC1_DATA, !(1 << KEYBOARD_LINE));
        outb(PIC2_DATA, 0xff);

        IDT.lock().set_handler(offset1 + KEYBOARD_LINE, isr_wrapper!(keyboard_handler));
    }
}

extern "C" fn keyboard_handler(_stack_frame: &ExceptionStackFrame) {
    unsafe {
        let val = inb(0x60);

        println!("{}", val);

        outb(PIC1_COMMAND, PIC_EOI);
    }
}

extern "C" fn pic1_interrupt_handler(_stack_frame: &ExceptionStackFrame) {
    unsafe {
        println!("\npic1_interrupt_handler\n");
    
        outb(PIC1_COMMAND, PIC_EOI);
    }
}

extern "C" fn pic2_interrupt_handler(_stack_frame: &ExceptionStackFrame) {
    unsafe {
        println!("\npic2_interrupt_handler\n");

        outb(PIC2_COMMAND, PIC_EOI);
    }
}
