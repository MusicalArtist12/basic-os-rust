use crate::println;

#[repr(C)]
struct InfoHeader {
    total_size: u32,
    reserved: u32,
}

#[repr(C)]
struct TagHeader {
    id: u32,
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u32)]
pub enum TagID {
    Terminal = 0,
    BootCommandLine = 1,
    BootLoaderName = 2,
    Modules = 3,
    MemInfo = 4,
    BIOSBootInfo = 5,
    MemMap = 6,
    VBEInfo = 7,
    FramebufferInfo = 8,
    ELFSymbols = 9,
    APMTable = 10,
    EFI32SysTablePtr = 11,
    EFI64SysTablePtr = 12,
    SMBIOSTables = 13,
    ACPIOldRSDP = 14,
    ACPINewRSDP = 15,
    NetworkingInfo = 16,
    EFIMemMap = 17,
    EFIBootServicesUncalled = 18,
    EFI32ImageHandlePtr = 19,
    EFI64ImageHandlePtr = 20,
    ImageLoadBasePhysAddr = 21
}

enum TagPtr {
    Terminal,
    MemInfo(*const MemInfo),
    MemMap(*const MemMap),
    BootCommandLine,
    BootLoaderName,
    Modules,
    BIOSBootInfo,
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
    Unknown(*const TagHeader),
}

fn get_tag(header: *const TagHeader) -> TagPtr {
    unsafe {
        
        
        /*
        match (*header).id as u32 {
            x if x == TagID::Terminal as u32 => {
                TagPtr::Terminal
            },
            x if x == TagID::MemInfo as u32 => {
                TagPtr::MemInfo(header as *const MemInfo)
            },
            x if x == TagID::MemMap as u32 => {
                TagPtr::MemMap(header as *const MemMap)
            },
            _ => {
                TagPtr::Unknown(header)
            },
        }
        */

        /*
        match (*header).id {
            TagID::Terminal => {
                TagPtr::Terminal
            },
            TagID::MemInfo => {
                TagPtr::MemInfo(header as *const MemInfo)
            },
            TagID::MemMap => {
                TagPtr::MemMap(header as *const MemMap)
            },
            _ => {
                println!("\tUnhandled id: {:?}", (*header).id);
                TagPtr::Unknown(header)
            },
        }
        */

        match (*header).id {
            0 => TagPtr::Terminal,
            1 => TagPtr::BootCommandLine,
            2 => TagPtr::BootLoaderName,
            3 => TagPtr::Modules,
            4 => TagPtr::MemInfo(header as *const MemInfo),
            5 => TagPtr::BIOSBootInfo,
            6 => TagPtr::MemMap(header as *const MemMap),
            7 => TagPtr::VBEInfo,
            8 => TagPtr::FramebufferInfo,
            9 => TagPtr::ELFSymbols,
            10 => TagPtr::APMTable,
            11 => TagPtr::EFI32SysTablePtr,
            12 => TagPtr::EFI64SysTablePtr,
            13 => TagPtr::SMBIOSTables,
            14 => TagPtr::ACPIOldRSDP,
            15 => TagPtr::ACPINewRSDP,
            16 => TagPtr::NetworkingInfo,
            17 => TagPtr::EFIMemMap,
            18 => TagPtr::EFIBootServicesUncalled,
            19 => TagPtr::EFI32ImageHandlePtr,
            20 => TagPtr::EFI64ImageHandlePtr,
            21 => TagPtr::ImageLoadBasePhysAddr,
            _ => TagPtr::Unknown(header)
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
                println!("mem_lower: {:#x}", (*info).mem_lower);
                println!("mem_upper: {:#x}\n",(*info).mem_upper);
            },
            TagPtr::MemMap(info) => {
                println!("MemMap Tag:");
                println!("entry_size: {:#x}", (*info).entry_size);
                println!("entry_version: {:#x}\n", (*info).entry_version);
            }
            TagPtr::Unknown(_) => {
                // println!("Unknown Tag @ {:#0x}, ID: {}, size: {}", info as u32, (*info).id as u32, (*info).size);
            }
            _ => {

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
        loop {
            
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