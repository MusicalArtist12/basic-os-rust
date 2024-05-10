pub mod idt;
pub mod irq;
use crate::kernel::io::basic_vga_driver::{STDOUT, CharAttr, Color};
use core::arch::asm;
use core::fmt::Write;
use idt::IDT;
use idt::ExceptionStackFrame;


#[no_mangle]
extern "C" fn stack_frame_handler(stack_frame: &ExceptionStackFrame) -> ! {
    unsafe {

        write!(*STDOUT.bypass_lock(), "\n{:#x?}", stack_frame).unwrap();

        //asm!("iretq");
    }
    loop {}
}

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                asm!(r#"
                    mov rdi, rsp
                    call {}
                "#, sym $name, options(noreturn))
            }
        }
        wrapper
    }};
}

macro_rules! quick_handler {
    ($str: literal, $name: ident, $additional_code: block) => {
        extern "C" fn $name(stack_frame: &ExceptionStackFrame) -> ! {
            unsafe {
                (*STDOUT.bypass_lock()).change_color(CharAttr::new(Color::White, Color::Blue));
                // (*STDOUT.bypass_lock()).clear_screen();
                write!(*STDOUT.bypass_lock(), "\n{}\n\n{:#x?}", $str, stack_frame).unwrap();
        
                //asm!("iretq");
            }
            $additional_code;
            loop { }
        }
    };
}

quick_handler!("EXCEPTION::division_by_zero", division_by_zero, { });
quick_handler!("EXCEPTION::single_step_interrupt", single_step_interrupt, { });
quick_handler!("EXCEPTION::non_maskable_interrupt", non_maskable_interrupt, { });
quick_handler!("EXCEPTION::breakpoint", breakpoint, { });
quick_handler!("EXCEPTION::overflow", overflow, { });
quick_handler!("EXCEPTION::bound_range_exceeded", bound_range_exceeded, { });
quick_handler!("EXCEPTION::invalid_opcode", invalid_opcode, { });
quick_handler!("EXCEPTION::coprocessor_not_available", coprocessor_not_available, { });
quick_handler!("EXCEPTION::double_fault", double_fault, { });
quick_handler!("EXCEPTION::coprocessor_segment_overrun", coprocessor_segment_overrun, { });
quick_handler!("EXCEPTION::invalid_tss", invalid_tss, { });
quick_handler!("EXCEPTION::segment_not_present", segment_not_present, { });
quick_handler!("EXCEPTION::stack_segment_fault", stack_segment_fault, { });
quick_handler!("EXCEPTION::general_protection_fault", general_protection_fault, { });
quick_handler!("EXCEPTION::page_fault", page_fault, { });
quick_handler!("EXCEPTION::reserved_exception", reserved_exception, { });
quick_handler!("EXCEPTION::floating_point_exception", floating_point_exception, { });
quick_handler!("EXCEPTION::alignment_check", alignment_check, { });
quick_handler!("EXCEPTION::machine_check", machine_check, { });
quick_handler!("EXCEPTION::simd_floating_point_exception", simd_floating_point_exception, { });
quick_handler!("EXCEPTION::virtualization_exception", virtualization_exception, { });
quick_handler!("EXCEPTION::control_protection_exception", control_protection_exception, { });


quick_handler!("EXCEPTION::unhandled_exception", unhandled_exception, { });

pub fn init_idt() {
    irq::mask_all();
    IDT.lock().set_handler(0, handler!(division_by_zero));
    IDT.lock().set_handler(1, handler!(single_step_interrupt));
    IDT.lock().set_handler(2, handler!(non_maskable_interrupt));
    IDT.lock().set_handler(3, handler!(breakpoint));
    IDT.lock().set_handler(4, handler!(overflow));
    IDT.lock().set_handler(5, handler!(bound_range_exceeded));
    IDT.lock().set_handler(6, handler!(invalid_opcode));
    IDT.lock().set_handler(7, handler!(coprocessor_not_available));
    IDT.lock().set_handler(8, handler!(double_fault));
    IDT.lock().set_handler(9, handler!(coprocessor_segment_overrun));
    IDT.lock().set_handler(10, handler!(invalid_tss));
    IDT.lock().set_handler(11, handler!(segment_not_present));
    IDT.lock().set_handler(12, handler!(stack_segment_fault));
    IDT.lock().set_handler(13, handler!(general_protection_fault));
    IDT.lock().set_handler(14, handler!(page_fault));
    IDT.lock().set_handler(15, handler!(reserved_exception));
    IDT.lock().set_handler(16, handler!(floating_point_exception));
    IDT.lock().set_handler(17, handler!(alignment_check));
    IDT.lock().set_handler(18, handler!(machine_check));
    IDT.lock().set_handler(19, handler!(simd_floating_point_exception));
    IDT.lock().set_handler(20, handler!(virtualization_exception));
    IDT.lock().set_handler(21, handler!(control_protection_exception));

    for i in 20..255 {
        IDT.lock().set_handler(i, handler!(unhandled_exception));
    }

    IDT.lock().load(); 
}