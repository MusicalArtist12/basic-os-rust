
use crate::println;

#[repr(C)]
struct InfoHeader {
    total_size: u32,
    reserved: u32,
}

#[repr(C)]
struct TagHeader {
    id: TagID,
    size: u32
}

#[repr(C)]
struct MemInfo {
    header: TagHeader,
    mem_lower: u32,
    mem_upper: u32
}

#[repr(C)]
struct MemMap {
    header: TagHeader,
    entry_size: u32,
    entry_version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TagID {
    Terminal = 0,
    MemInfo = 4,
    MemMap = 6,
    Unknown = u32::MAX,
}

enum TagPtr {
    Terminal,
    MemInfo(*const MemInfo),
    MemMap(*const MemMap),
    Unknown(*const TagHeader),
}

fn get_tag(header: *const TagHeader) -> TagPtr {
    unsafe {
        match (*header).id {
            TagID::Terminal => {
                TagPtr::Terminal
            },
            TagID::MemInfo => {
                TagPtr::MemInfo(header as *const MemInfo)
            },
            // TagID::MemMap => {
            //     TagPtr::MemMap(header as *const MemMap)
            // },
            _ => {
                TagPtr::Unknown(header)
            },
        }
    }
}

impl TagPtr {
    pub unsafe fn printout(&self) {
        match *self {
            TagPtr::Terminal => {
                println!("Terminal Tag");
            },
            TagPtr::MemInfo(info) => {
                println!("MemInfo Tag:");
                println!("mem_lower: {}", (*info).mem_lower);
                println!("mem_upper: {}\n",(*info).mem_upper);
            },
            TagPtr::MemMap(info) => {
                println!("MemMap Tag:");
                println!("entry_size: {}", (*info).entry_size);
                println!("entry_version: {}\n", (*info).entry_version);
            }
            TagPtr::Unknown(info) => {
                println!("Unknown Tag @ {:#0x}, ID: {}, size: {}", info as u32, (*info).id as u32, (*info).size);
            }
        }
    }
}

pub fn read_multiboot(addr: u32) {
    
    unsafe {
        let header = &*(addr as *const InfoHeader);

        if header.total_size & 0b111 != 0 {
            println!("size is not a multiple of 8!");
        }

        println!("Header: ");
        println!("addr: {:#x}", addr as u32);
        println!("size:      {}", header.total_size);
        // println!("reserved:  {:#b}\n", header.reserved);

        let mut ptr = addr + 8;
        for _ in 0..10 {
            
            let tag_header = ptr as *const TagHeader;
            
            let tag = get_tag(tag_header);

            // println!("addr: {:#x}", ptr);

            match tag {
                TagPtr::Terminal => {
                    tag.printout();
                    break;
                }
                _ => {
                    tag.printout();
                }
            }

            ptr += (*tag_header).size;
                
            // get the nearest multiple of 8
            while ptr % 8 != 0 {
                ptr += 1;
            }

        }
    }
    
}