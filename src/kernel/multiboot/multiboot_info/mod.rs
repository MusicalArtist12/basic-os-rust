pub mod tags;
pub use tags::*;

use core::fmt::Debug;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MultibootInfoHeader {
    pub size: u32,
    reserved: u32,
    tag_start: u8
}

#[derive(Copy, Clone, Debug)]
pub struct MultibootInfo<'a> {
    pub header: &'a MultibootInfoHeader,
}

pub struct TagIter {
    current_section: *const u8,
    remaining: u32,    
}

impl Iterator for TagIter {
    type Item = TagPtr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            unsafe {
                let header = HeaderTag::read(self.current_section);
                let tag = TagPtr::new(header, self.current_section);
                
                let mut increment = header.size;
                while (increment % 8) != 0 {
                    increment += 1;
                }

                if increment == 0 {
                    return None;
                }

                self.current_section = self.current_section.offset(increment as isize);
                self.remaining -= increment;

                Some(tag)
            }
        }
        else {
            None
        }
    }
}

impl<'a> MultibootInfo<'a> {
    pub const fn new(addr: usize) -> Self {
        let info: MultibootInfo = MultibootInfo {
            header: unsafe { &*(addr as *const MultibootInfoHeader) },
        };

        info
    }

    pub fn tags(&self) -> TagIter {
        TagIter {
            current_section: &self.header.tag_start,
            remaining: self.header.size
        }
    }

    pub fn get_tag(&self, id: TagID) -> Option<TagPtr> {
        self.tags().find(|&x| {
            x.id() == id
        })
    }
}
