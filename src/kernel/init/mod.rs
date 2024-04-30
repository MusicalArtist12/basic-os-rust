use core::arch::global_asm;
mod multiboot;


global_asm!(include_str!("bootloader.asm"));

global_asm!(r#"
.code32
.section .init.text, "ax", @progbits
long_jump:
    ljmp  $gdt64_code_seg, $_start
"#, options(att_syntax));

static HELLO: &[u8] = b"Hello World!     ";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}