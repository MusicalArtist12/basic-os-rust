use core::{cmp::Ordering, fmt::Debug};

use super::HeaderTag;
use crate::println;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELFSymbols {
    pub header: HeaderTag,
    num: u32,
    entsize: u32,
    shndx: u32,
    entries: u8
}


#[derive(Copy, Clone, Debug)]
#[repr(u32)]
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
    OutOfBounds
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ELFSectionHeader {
    entsize: u32,
    ptr: *const u8
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct ELFSectionIter {
    current_section: *const u8,
    remaining: u32,
    entsize: u32
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
            println!("addr: {:#x} size: {:#x} flags {:#x} type {:#?}", i.addr(), i.size(), i.flags(), i.typ());
        }
    }
}

impl Debug for ELFSymbols {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ELFSymbols")
            .field("header", &self.header)
            .field("num", &self.num)
            .field("entsize", &self.entsize)
            .field("shndx", &self.shndx)
            .finish()
    }
}

impl ELFSectionHeader {
    fn cast_as_32(&self) -> &'static ELF32SectionHeader {
        return unsafe { &*(self.ptr as *const ELF32SectionHeader) }
    }

    fn cast_as_64(&self) -> &'static ELF64SectionHeader {
        return unsafe { &*(self.ptr as *const ELF64SectionHeader) }
    }

    pub fn name(&self) -> u32 {
        match self.entsize {
            32 => { self.cast_as_32().name },
            64 => { self.cast_as_64().name },
            _ => { panic!() }  
        }
    }
    
    pub fn addr(&self) -> u64 {
        match self.entsize {
            32 => { self.cast_as_32().addr as u64 },
            64 => { self.cast_as_64().addr },
            _ => { panic!() }  
        }
    }

    pub fn typ(&self) -> ELFSectionType {
        let typ = match self.entsize {
            32 => { self.cast_as_32().typ },
            64 => { self.cast_as_64().typ },
            _ => { panic!() }  
        };

        if typ >= ELFSectionType::OutOfBounds as u32 {
            panic!("ELFSectionType out of bounds!");
        }

        unsafe { core::mem::transmute::<u32, ELFSectionType>(typ) }
    }

    pub fn flags(&self) -> u64 {
        match self.entsize {
            32 => { self.cast_as_32().flags as u64 },
            64 => { self.cast_as_64().flags },
            _ => { panic!() }  
        }
    }

    pub fn size(&self) -> u64 {
        match self.entsize {
            32 => { self.cast_as_32().flags as u64 },
            64 => { self.cast_as_64().flags },
            _ => { panic!() }  
        }
    }
}

impl PartialOrd for ELFSectionHeader {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self.addr() > other.addr() {
            Some(Ordering::Greater)
        }
        else if self.addr() == other.addr() {
            Some(Ordering::Equal)
        }
        else {
            Some(Ordering::Less)
        }
    }
}

impl Ord for ELFSectionHeader {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("none")
    }
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