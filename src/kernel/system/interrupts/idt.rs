use core::arch::asm;
use crate::kernel::system::gdt::gdt64_code_offset;
use crate::kernel::sync::mutex::Mutex;
use crate::println;

pub type HandlerFunc = extern "C" fn() -> !;

pub struct Idt([Entry; 256]);

#[repr(C, packed)]
struct Idtr {
    limit: u16,
    base: u64
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: u16,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

#[derive(Clone, Copy)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn new() -> Self { 
        let mut options = EntryOptions(0x0E00);
        options.set_present(true);
        options.disable_interrupts(true);
        options
    }

    pub fn set_present(&mut self, present: bool) {
        self.0 = self.0 & !(1 << 15);
        if present {
            self.0 = self.0 | (1 << 15);
        }
    }

    pub fn disable_interrupts(&mut self, disable: bool) {
        self.0 = self.0 & !(1 << 8);
        if !disable {
            self.0 = self.0 | (1 << 8);
        }
    }

    pub fn set_privilege_level(&mut self, dpl: u16) {
        self.0 = self.0 & !(0b11 << 13);
        self.0 = self.0 | ((dpl & 0b11) << 13);
    }

    pub fn set_stack_index(&mut self, index: u16) {
        self.0 = self.0 & !(0b11);
        self.0 = self.0 | (index & 0b11);
    }
}

impl Entry {

    fn new(handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector: gdt64_code_offset(),
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16) as u16,
            pointer_high: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0
        }
    }

    const fn unhandled() -> Self {
        Entry {
            gdt_selector: 0,
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: EntryOptions(0),
            reserved: 0,
        }
    }
}

impl Idt {
    pub const fn new() -> Idt {
        Idt([Entry::unhandled(); 256])
    }

    pub fn set_handler(&mut self, entry: u8, handler: HandlerFunc) -> &mut EntryOptions {
        // println!("setting {} for {}", entry, (self as *const Idt) as u64);
        
        self.0[entry as usize] = Entry::new(handler);
        &mut self.0[entry as usize].options
    }

    pub fn load(& self) {
        
        IDTR.lock().get_mut().base = self as *const _ as u64;
        IDTR.lock().get_mut().limit = core::mem::size_of::<Self>() as u16 - 1;  

        println!("loading addr: {:#x}", self as *const _ as u64);
        
        unsafe {    
            let addr = IDTR.lock().get_ptr();
            println!("IDTR addr: {:#x}", addr as u64);

            asm!(r#"
                lidt [{}]
                sti
            "#, in(reg) addr);
        }
    }
}

static IDTR: Mutex<Idtr> = Mutex::new(Idtr {limit: 0, base: 0});
pub static IDT: Mutex<Idt> = Mutex::new(Idt::new());