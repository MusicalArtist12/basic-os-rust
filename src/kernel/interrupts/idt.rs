use super::entry::{Entry, EntryOptions};
use super::HandlerFunc;
use super::IDTR;

use core::arch::asm;

pub struct Idt([Entry; 256]);

#[repr(C, packed)]
pub struct Idtr {
    limit: u16,
    base: u64
}

impl Idtr {
    pub const fn new() -> Self {
        Idtr {limit: 0, base: 0}
    }
}

impl Idt {
    pub const fn new() -> Self {
        Idt([Entry::unhandled(); 256])
    }

    pub fn set_handler(&mut self, entry: u8, handler: HandlerFunc) -> &mut EntryOptions {
        self.0[entry as usize] = Entry::new(handler);
        &mut self.0[entry as usize].options
    }

    pub fn load(& self) {
        IDTR.lock().base = self as *const _ as u64;
        IDTR.lock().limit = core::mem::size_of::<Self>() as u16 - 1;  

        unsafe {    
            let addr = IDTR.lock().get_ptr();
            // println!("IDTR addr: {:#x}", addr as u64);

            asm!(r#"
                lidt [{}]
            "#, in(reg) addr);
        }
    }
}
