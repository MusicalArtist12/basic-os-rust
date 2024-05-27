pub mod tags;
pub use tags::*;

use core::fmt::Debug;

#[derive(Copy, Clone)]
enum TagPtr {
    MemInfo(&'static MemInfo),
    MemMap(&'static MemMap),
    ELFSymbols(&'static ELFSymbols),
    Terminal,
    Unhandled(&'static HeaderTag)
}

impl TagPtr {
    pub unsafe fn new(header: &HeaderTag, addr: *const u8) -> Self {
        match header.typ() {
            TagID::Terminal => {
                TagPtr::Terminal
            },
            TagID::MemInfo => {
                TagPtr::MemInfo(&*(addr as *const MemInfo))
            },
            TagID::MemMap => {
                TagPtr::MemMap(&*(addr as *const MemMap))
            },
            TagID::ELFSymbols => {
                TagPtr::ELFSymbols(&*(addr as *const ELFSymbols))
            }
            _ => {
                TagPtr::Unhandled(&*(addr as *const HeaderTag))
            }
        }
    }

    pub fn id(&self) -> TagID {
        match self {
            TagPtr::MemInfo(_) => { TagID::MemInfo },
            TagPtr::MemMap(_) => { TagID::MemMap },
            TagPtr::ELFSymbols(_) => { TagID::ELFSymbols },
            TagPtr::Terminal => { TagID::Terminal },
            Self::Unhandled(header) => { 
                header.typ()
            },
        }
    }
}

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

struct TagIter {
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

    fn tags(&self) -> TagIter {
        TagIter {
            current_section: &self.header.tag_start,
            remaining: self.header.size
        }
    }

    fn get_tag(&self, id: TagID) -> Option<TagPtr> {
        self.tags().find(|&x| {
            x.id() == id
        })
    }

    pub fn memmap(&self) -> Option<&MemMap> {
        if let Some(TagPtr::MemMap(mem_map)) = self.get_tag(TagID::MemMap) {
            Some(mem_map)
        }
        else {
            None
        }
    }

    pub fn meminfo(&self) -> Option<&MemInfo> {
        if let Some(TagPtr::MemInfo(mem_info)) = self.get_tag(TagID::MemInfo) {
            Some(mem_info)
        }
        else {
            None
        }
    }

    pub fn elfsymbols(&self) -> Option<&ELFSymbols> {
        if let Some(TagPtr::ELFSymbols(mem_info)) = self.get_tag(TagID::ELFSymbols) {
            Some(mem_info)
        }
        else {
            None
        }
    }
}
