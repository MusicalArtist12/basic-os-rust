pub mod idt;
pub mod irq;
use crate::println;
use core::arch::asm;
use idt::IDT;
use idt::ExceptionStackFrame;

#[naked]
extern "C" fn division_by_zero() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn single_step_interrupt() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn non_maskable_interrupt() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn breakpoint() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn overflow() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn bound_range_exceeded() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn invalid_opcode() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn coprocessor_not_available() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn double_fault() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn invalid_tss() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn segment_not_present() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn stack_seg_fault() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn general_protection() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}

#[naked]
extern "C" fn page_fault() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }    
}


#[naked]
extern "C" fn reserved() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }
}


#[naked]
extern "C" fn unhandled() -> ! {
    unsafe {
        asm!("mov rdi, rsp; call stack_frame_handler", options(noreturn))
    }
}

#[no_mangle]
extern "C" fn stack_frame_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("\n{:#?}", stack_frame);

    loop {}
}

pub fn init_idt() {
    irq::mask_all();
    
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