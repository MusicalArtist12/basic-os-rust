pub mod multiboot_info_tags;
pub use multiboot_info_tags::*;

use core::mem::size_of;
use crate::println;

#[repr(C)]
pub struct MultibootInfoHeader {
    size: u32,
    reserved: u32,
}

pub struct MultibootInfo<'a> {
    header: &'a MultibootInfoHeader,
    tags: [TagPtr; 2]
}

impl<'a> MultibootInfo<'a> {
    pub unsafe fn new(addr: usize) -> Self {
        let mut info: MultibootInfo = MultibootInfo {
            header: &*(addr as *const MultibootInfoHeader),
            tags: [TagPtr::None; 2]
        };

        let mut ptr = addr + size_of::<MultibootInfoHeader>();
        let mut count = 0;

        loop {
            let header = HeaderTag::read(ptr);
            let tag = TagPtr::new(header, ptr);

            match tag {
                TagPtr::None => {

                },
                TagPtr::Terminal => {
                    break;
                },
                _ => {
                    info.tags[count] = tag;
                    count = count + 1;
                }
            }

            if count == info.tags.len() {
                break;
            }

            ptr = ptr + header.size as usize;
            
            while (ptr % 8) != 0 {
                ptr = ptr + 1;
            }
        }

        println!("count: {}", count);

        info
    }

    pub fn log_tags(&self) {
        for i in self.tags {
            println!("{:#?}", i);

            match i {
                TagPtr::MemMap(map) => {
                    unsafe { map.print_map() };
                },
                _ => {
                    
                }
            }
        }
    }
}