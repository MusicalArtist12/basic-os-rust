use super::HeaderTag;
use crate::println;
use core::mem::size_of;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemMap {
    pub header: HeaderTag,
    entry_size: u32,
    pub entry_version: u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryArea {
    pub base_addr: u64,
    pub length: u64,
    pub typ: u32, // 1 == available, 3 == ACPI, 4 == reserved, 5 == defective, all else == reserved
    reserved: u32
}


pub struct MemoryAreaIter {
    current_section: *const u8,
    remaining: u32,
}

impl Iterator for MemoryAreaIter {
    type Item = MemoryArea;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let entry = unsafe { *(self.current_section as *const MemoryArea) };

            self.current_section = unsafe { self.current_section.offset(size_of::<MemoryArea>() as isize)};
            self.remaining -= 1;

            Some(entry)
        }
        else {
            None
        }
    }
}

impl MemMap {
    pub fn num_entries(&self) -> usize {
        (self.header.size / self.entry_size) as usize
    }

    pub fn print_map(&self) {
        let slice = self.get_entries();

        for i in slice {
            if i.typ == 1 {
                println!("addr: {:>#16x} length: {:>#16x} type: {:>8}", i.base_addr, i.length, i.typ);
            }
        }
    }

    pub fn get_entries(&self) -> MemoryAreaIter {
        MemoryAreaIter {
            current_section: (((self as *const _) as usize) + size_of::<Self>()) as *const u8,
            remaining: self.num_entries() as u32
        }
    }
}

