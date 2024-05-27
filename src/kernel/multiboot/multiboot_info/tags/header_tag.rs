use core::fmt::Debug;

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeaderTag {
    typ_val: u32,
    pub size: u32
}

impl HeaderTag {
    pub unsafe fn read(addr: *const u8) -> &'static Self {
        &*(addr as *const HeaderTag)
    }

    pub fn typ(&self) -> TagID {
        if self.typ_val >= TagID::OutOfBounds as u32 {
            panic!("unknown multiboot info tag {}", self.typ_val );
        };

        unsafe { core::mem::transmute::<u32, TagID>(self.typ_val) }
    }
}

impl Debug for HeaderTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let tag_id = self.typ();
        
        f.debug_struct("HeaderTag")
            .field("typ", &tag_id)
            .field("size", &self.size)
            .finish()
    }
}