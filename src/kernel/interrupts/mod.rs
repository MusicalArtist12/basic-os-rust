mod idt;
mod entry;

use core::arch::asm;
use idt::{Idt, Idtr};
use crate::kernel::sync::mutex::Mutex;

pub type HandlerFunc = extern "C" fn() -> !;

#[repr(u8)]
pub enum CPUExceptions {
    DivisionByZero,
    SingleStepInterrupt,
    NonMaskableInterrupt,
    Breakpoint,
    Overflow,
    BoundRangeExceeded,
    InvalidOpcode,
    CoprocessorNotAvailable,
    DoubleFault,
    CoprocessorSegmentOverrun,
    InvalidTSS,
    SegmentNotPresent,
    StackSegmentFault,
    GeneralProtectionFault,
    PageFault,
    Reserved,
    FloatingPointException,
    AlignmentCheck,
    MachineCheck,
    SIMDFloatingPointException,
    VirtualizationException,
    ControlProtectionException
}

#[macro_export]
macro_rules! isr_wrapper {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!(r#"
                    push rax
                    push rcx
                    push rdx
                    push rsi
                    push rdi
                    push r8
                    push r9
                    push r10
                    push r11

                    mov rdi, rsp
                    sub rsp, 8
                    call {}

                    add rsp, 8

                    pop r11
                    pop r10
                    pop r9
                    pop r8
                    pop rdi
                    pop rsi
                    pop rdx
                    pop rcx
                    pop rax
   
                    iretq
                "#, sym $name, options(noreturn));
            }
        }
        wrapper
    }};
}


#[derive(Debug)]
#[repr(C)]
pub struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}


static IDTR: Mutex<Idtr> = Mutex::new(Idtr::new());
pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());

extern "C" fn double_fault_handler(stack_frame: &ExceptionStackFrame) {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "C" fn general_protection_handler(stack_frame: &ExceptionStackFrame) {
    panic!("EXCEPTION: GENERAL_PROTECTION\n{:#?}", stack_frame);
}

extern "C" fn page_fault_handler(stack_frame: &ExceptionStackFrame) {
    panic!("EXCEPTION: PAGE_FAULT\n{:#?}", stack_frame);
}

pub fn load_interrupt_handlers() {
    IDT.lock().set_handler(CPUExceptions::DoubleFault as u8, isr_wrapper!(double_fault_handler));
    IDT.lock().set_handler(CPUExceptions::GeneralProtectionFault as u8, isr_wrapper!(general_protection_handler));
    IDT.lock().set_handler(CPUExceptions::PageFault as u8, isr_wrapper!(page_fault_handler));

    IDT.lock().load(); 
}
