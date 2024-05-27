pub mod elf_symbols;
pub mod meminfo;
pub mod memmap;
pub mod header_tag;

use core::fmt::Debug;

pub use elf_symbols::*;
pub use meminfo::*;
pub use memmap::*;
pub use header_tag::*;


#[derive(Copy, Clone)]
pub enum TagPtr {
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

impl Debug for TagPtr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TagPtr::Unhandled(info) => {
                f.debug_struct("Unhandled").field("info", info).finish()
            },
            TagPtr::Terminal => {
                f.write_str("Terminal")
            },
            TagPtr::MemInfo(info) => {
                f.debug_struct("MemInfo").field("info", info).finish()
            },
            TagPtr::MemMap(info) => {
                f.debug_struct("MemMap").field("info", info).finish()
            },
            TagPtr::ELFSymbols(info) => {
                f.debug_struct("ELFSymbols").field("info", info).finish()
            }
        }

    }
}

