use super::HeaderTag;
use crate::println;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ELFSymbols {
    pub header: HeaderTag,
    num: u32,
    entsize: u32,
    shndx: u32,
    entries: u8
}

pub enum ELFSectionType {
    Unused,
    ProgramSection,
    LinkerSymbolTable,
    StringTable,
    RelaRelocation,
    SymbolHashTable,
    DynamicLinkingTable,
    Note,
    Uninitialized,
    RelRelocation,
    Reserved,
    DynamicLoaderSymbolTable,
    EnvironmentSpecific,
    ProcessorSpecific,
}

pub struct ELFSectionHeader {
    entsize: u32,
    ptr: *const u8
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct ELF32SectionHeader {
    name: u32,
    typ: u32,
    flags: u32,
    addr: u32,
    offset: u32,
    size: u32,
    link: u32,
    info: u32,
    addralign: u32,
    entsize: u32
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
struct ELF64SectionHeader {
    name: u32,
    typ: u32,
    flags: u64,
    addr: u64,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    addralign: u64,
    entsize: u64
}

impl ELFSectionHeader {
    fn cast_as_32(&self) -> &'static ELF32SectionHeader {
        return unsafe { &*(self.ptr as *const ELF32SectionHeader) }
    }

    fn cast_as_64(&self) -> &'static ELF64SectionHeader {
        return unsafe { &*(self.ptr as *const ELF64SectionHeader) }
    }

    fn name(&self) -> u32 {
        match self.entsize {
            32 => { self.cast_as_32().name },
            64 => { self.cast_as_64().name },
            _ => { panic!() }  
        }
    }
    
    fn addr(&self) -> u64 {
        match self.entsize {
            32 => { self.cast_as_32().addr as u64 },
            64 => { self.cast_as_64().addr },
            _ => { panic!() }  
        }
    }

    fn typ(&self) -> u32 {
        match self.entsize {
            32 => { self.cast_as_32().typ },
            64 => { self.cast_as_64().typ },
            _ => { panic!() }  
        }
    }

    fn flags(&self) -> u64 {
        match self.entsize {
            32 => { self.cast_as_32().flags as u64 },
            64 => { self.cast_as_64().flags },
            _ => { panic!() }  
        }
    }

    fn size(&self) -> u64 {
        match self.entsize {
            32 => { self.cast_as_32().flags as u64 },
            64 => { self.cast_as_64().flags },
            _ => { panic!() }  
        }
    }
}

pub struct ELFSectionIter {
    current_section: *const u8,
    remaining: u32,
    entsize: u32
}

impl Iterator for ELFSectionIter {
    type Item = ELFSectionHeader;

    fn next(&mut self) -> Option<ELFSectionHeader> {
        if self.remaining > 0 {
            let section = ELFSectionHeader {
                entsize: self.entsize,
                ptr: self.current_section
            };

            self.current_section = unsafe { self.current_section.offset(self.entsize as isize)};
            self.remaining -= 1;

            return Some(section);
        }
        else {
            return None;
        }
    }
}

impl ELFSymbols {
    pub fn get_section_headers(&self) -> ELFSectionIter {
        ELFSectionIter {
            current_section: &self.entries,
            remaining: self.num,
            entsize: self.entsize
        }
    }

    pub fn print_headers(&self) {
        for i in self.get_section_headers() {
            println!("addr: {:#x} size: {:#x} flags {:#x}", i.addr(), i.size(), i.flags());
        }
    }

}