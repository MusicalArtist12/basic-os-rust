use super::ENTRY_COUNT;
use core::ops::{Index, IndexMut};

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
    number: usize
}

pub struct Entry(u64);

pub type EntryOption = u64;

const PRESENT: EntryOption =        1;
const WRITABLE: EntryOption =       1 << 2;
const WRITE_THROUGH: EntryOption =  1 << 3;
const DISABLE_CACHE: EntryOption =  1 << 4;
const ACCESSED: EntryOption =       1 << 5;
const DIRTY: EntryOption =          1 << 6;
const HUGE_PAGE: EntryOption =      1 << 7;
const GLOBAL: EntryOption =         1 << 8;
const NO_EXECUTE: EntryOption =     1 << 63;

impl Entry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn set_bits(&mut self, options: EntryOption) {
        self.0 = self.0 | options 
    }

    pub fn clear_bits(&mut self, options: EntryOption) {
        self.0 = self.0 & !options
    }

    pub fn new(options: EntryOption) -> Self {
        Entry(options)
    }

    pub fn is_set(&self, options: EntryOption) -> bool {
        (self.0 & options) == options
    }

}

pub struct PageTable {
    entries: [Entry; ENTRY_COUNT]
}

impl Index<usize> for PageTable {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

impl PageTable {
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}