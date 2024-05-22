pub mod multiboot_info_tags;
pub use multiboot_info_tags::*;
pub mod elf_tag;
pub use elf_tag::*;

use core::mem::size_of;
use crate::println;

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum TagID {
    Terminal = 0,
    BootCommandLine,
    BootLoaderName,
    Modules,
    MemInfo,
    BIOSBootInfo,
    MemMap,
    VBEInfo,
    FramebufferInfo,
    ELFSymbols,
    APMTable,
    EFI32SysTablePtr,
    EFI64SysTablePtr,
    SMBIOSTables,
    ACPIOldRSDP,
    ACPINewRSDP,
    NetworkingInfo,
    EFIMemMap,
    EFIBootServicesUncalled,
    EFI32ImageHandlePtr,
    EFI64ImageHandlePtr,
    ImageLoadBasePhysAddr,
    OutOfBounds
}

#[derive(Copy, Clone, Debug)]
pub enum TagPtr {
    MemInfo(&'static MemInfo),
    MemMap(&'static MemMap),
    ELFSymbols(&'static ELFSymbols),
    Terminal,
    None
}

impl TagPtr {
    pub unsafe fn new(header: &HeaderTag, addr: usize) -> Self {

        match header.typ {
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
                TagPtr::None
            }
        }
    }

}

#[repr(C)]
pub struct MultibootInfoHeader {
    size: u32,
    reserved: u32,
}

pub struct MultibootInfo<'a> {
    header: &'a MultibootInfoHeader,
    tags: [TagPtr; 100]
}

impl<'a> MultibootInfo<'a> {
    pub unsafe fn new(addr: usize) -> Self {
        let mut info: MultibootInfo = MultibootInfo {
            header: &*(addr as *const MultibootInfoHeader),
            tags: [TagPtr::None; 100]
        };

        if info.header.reserved != 0 {
            panic!("multiboot header may be currupted");
        }

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
            
            match i {
                TagPtr::MemMap(map) => {
                    println!("{:#?}", i);
                    map.print_map()
                },
                TagPtr::ELFSymbols(symbols) => {
                    println!("{:#?}", i);
                    symbols.print_headers();
                }
                TagPtr::None => {

                },
                _ => {
                    // println!("{:#?}", i);
                }
            }
        }
    }
}