pub mod idt;
pub mod irq;
use crate::{kernel::io::basic_vga_driver::STDOUT, println};
use core::fmt::Write;
use core::arch::asm;
use idt::IDT;

extern "C" fn division_by_zero() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\ndivision_by_zero\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn single_step_interrupt() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\nsingle_step_interrupt\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn non_maskable_interrupt() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\nnon_maskable_interrupt\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn breakpoint() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\nbreakpoint\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn overflow() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\noverflow\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn bound_range_exceeded() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\nbound_range_exceeded\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn invalid_opcode() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\ninvalid_opcode\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn coprocessor_not_available() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\ncoprocessor_not_available\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn double_fault() -> ! {
    unsafe {
        write!(*STDOUT.bypass_lock(), "\ndouble_fault\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn invalid_tss() -> ! {
    unsafe {
        write!(*STDOUT.bypass_lock(), "\ninvalid_tss\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn segment_not_present() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\nsegment_not_present\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn stack_seg_fault() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\nstack_seg_fault\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn general_protection() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\ngeneral_protection\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn page_fault() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\npage_fault\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn reserved() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\nreserved\n");
        // asm!("iretq")
    }
    loop { }
}

extern "C" fn unhandled() -> ! {
    
    unsafe {
        write!(*STDOUT.bypass_lock(), "\nunhandled\n");
        // asm!("iretq")
    }
    loop { }
}

pub fn init_idt() {
    IDT.lock().set_handler(0, division_by_zero);
    IDT.lock().set_handler(1, single_step_interrupt);
    IDT.lock().set_handler(2, non_maskable_interrupt);
    IDT.lock().set_handler(3, breakpoint);
    IDT.lock().set_handler(4, overflow);
    IDT.lock().set_handler(5, bound_range_exceeded);
    IDT.lock().set_handler(6, invalid_opcode);
    IDT.lock().set_handler(7, coprocessor_not_available);
    IDT.lock().set_handler(8, double_fault);
    IDT.lock().set_handler(0x0A, invalid_tss);
    IDT.lock().set_handler(0x0B, segment_not_present);
    IDT.lock().set_handler(0x0C, stack_seg_fault);
    IDT.lock().set_handler(0x0D, general_protection);
    IDT.lock().set_handler(0x0E, page_fault);
    IDT.lock().set_handler(0x0F, reserved);

    for i in 16..255 {
        IDT.lock().set_handler(i, unhandled);
    }

    IDT.lock().load(); 
}