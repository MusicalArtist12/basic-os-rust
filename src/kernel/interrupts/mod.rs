mod idt;
mod entry;

use idt::{Idt, Idtr};
use core::arch::asm;
use super::sync::mutex::Mutex;
use crate::println;

pub type HandlerFunc = extern "C" fn() -> !;

static IDTR: Mutex<Idtr> = Mutex::new(Idtr::new());
pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());

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


extern "C" fn double_fault_handler(stack_frame: &ExceptionStackFrame) {
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}\n", stack_frame);
    loop {};
}


pub fn load_interrupt_handlers() {
    IDT.lock().set_handler(CPUExceptions::DoubleFault as u8, isr_wrapper!(double_fault_handler));

    IDT.lock().load(); 

}
