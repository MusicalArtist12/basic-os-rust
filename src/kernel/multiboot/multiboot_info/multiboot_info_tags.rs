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
    Terminal,
    None
}

impl TagPtr {
    pub unsafe fn new(header: &HeaderTag, addr: usize) -> Self {

        match header.tag_type {
            TagID::Terminal => {
                TagPtr::Terminal
            },
            TagID::MemInfo => {
                TagPtr::MemInfo(&*(addr as *const MemInfo))
            },
            TagID::MemMap => {
                TagPtr::MemMap(&*(addr as *const MemMap))
            }
            _ => {
                TagPtr::None
            }
        }
    }

}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct HeaderTag {
    pub tag_type: TagID,
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

#[derive(Copy, Clone, Debug)]
pub struct MemMap {
    pub header: HeaderTag,
    entry_size: u32,
    pub entry_version: u32
}

#[derive(Copy, Clone, Debug)]
pub struct MemMapEntry {
    base_addr: u64,
    length: u64,
    entry_type: u32, // 1 == available, 3 == ACPI, 4 == reserved, 5 == defective, all else == reserved
    reserved: u32
}

impl MemMap {
    pub unsafe fn num_entries(&self) -> usize {
        (self.header.size / self.entry_size) as usize
    }

    pub unsafe fn print_map(&self) {
        let slice = self.get_entries();

        println!("{:>16}    {:>16}  {:>8}", "address", "length", "type");
        for i in slice {
            println!("{:>#16x}    {:>#16x}  {:>8}", i.base_addr, i.length, i.entry_type);
        }
    }

    pub unsafe fn get_entries(&self) -> &'static [MemMapEntry] {
        let ptr = ((self as *const _) as usize) + size_of::<MemMap>();
        let num = self.num_entries();

        core::slice::from_raw_parts(ptr as *const MemMapEntry, num.try_into().unwrap())
    }
}