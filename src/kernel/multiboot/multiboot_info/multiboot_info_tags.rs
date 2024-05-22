use core::mem::size_of;
use crate::println;

use super::TagID;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct HeaderTag {
    pub typ: TagID,
    pub size: u32
}

impl HeaderTag {
    pub unsafe fn read(addr: usize) -> &'static Self {
        struct SafeTagHeader {
            tag_type: u32,
            size: u32
        }

        if (&*(addr as *const SafeTagHeader)).tag_type >= TagID::OutOfBounds as u32 {
            panic!("multiboot info tag larger than expected {}", (&*(addr as *const SafeTagHeader)).tag_type );
        }

        &*(addr as *const HeaderTag)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MemInfo {
    pub header: HeaderTag,
    mem_lower: u32,
    mem_upper: u32 
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemMap {
    pub header: HeaderTag,
    entry_size: u32,
    pub entry_version: u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemMapEntry {
    base_addr: u64,
    length: u64,
    typ: u32, // 1 == available, 3 == ACPI, 4 == reserved, 5 == defective, all else == reserved
    reserved: u32
}

pub struct MemMapEntryIter {
    current_section: *const u8,
    remaining: u32,
}

impl Iterator for MemMapEntryIter {
    type Item = MemMapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let entry = unsafe { *(self.current_section as *const MemMapEntry) };

            self.current_section = unsafe { self.current_section.offset(size_of::<MemMapEntry>() as isize)};
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

    pub fn get_entries(&self) -> MemMapEntryIter {
        MemMapEntryIter {
            current_section: (((self as *const _) as usize) + size_of::<Self>()) as *const u8,
            remaining: self.num_entries() as u32
        }
    }
}

