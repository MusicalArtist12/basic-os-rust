pub mod idt;
pub mod irq;
use crate::println;
use idt::IDT;

extern "C" fn division_by_zero() -> ! {
    
    println!("\ndivision_by_zero\n");
    
    loop { }
}

extern "C" fn single_step_interrupt() -> ! {
    
    println!("\nsingle_step_interrupt\n");
    
    loop { }
}

extern "C" fn non_maskable_interrupt() -> ! {
    
    println!("\nnon_maskable_interrupt\n");
    
    loop { }
}

extern "C" fn breakpoint() -> ! {
    
    println!("\nbreakpoint\n");
    
    loop { }
}

extern "C" fn overflow() -> ! {
    
    println!("\noverflow\n");
    
    loop { }
}

extern "C" fn bound_range_exceeded() -> ! {
    
    println!("\nbound_range_exceeded\n");
    
    loop { }
}

extern "C" fn invalid_opcode() -> ! {
    
    println!("\ninvalid_opcode\n");
    
    loop { }
}

extern "C" fn coprocessor_not_available() -> ! {
    
    println!("\ncoprocessor_not_available\n");
    
    loop { }
}

extern "C" fn double_fault() -> ! {
    println!("\ndouble_fault\n");
    
    loop { }
}

extern "C" fn invalid_tss() -> ! {
    println!("\ninvalid_tss\n");
    
    loop { }
}

extern "C" fn segment_not_present() -> ! {
    
    println!("\nsegment_not_present\n");
    
    loop { }
}

extern "C" fn stack_seg_fault() -> ! {
    
    println!("\nstack_seg_fault\n");
    
    loop { }
}

extern "C" fn general_protection() -> ! {
    
    println!("\ngeneral_protection\n");
    
    loop { }
}

extern "C" fn page_fault() -> ! {
    
    println!("\npage_fault\n");
    
    loop { }
}

extern "C" fn reserved() -> ! {
    
    println!("\nreserved\n");
    
    loop { }
}

extern "C" fn unhandled() -> ! {
    
    println!("\nunhandled\n");
    
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