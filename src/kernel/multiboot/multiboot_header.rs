const HEADER_MAGIC: u32 = 0xE85250D6;
const HEADER_ARCH:  u32 = 0;

use super::{REQUESTS, RequestType};
use core::mem::size_of;

#[repr(C, align(8))]
struct HeaderTag {
    tag: u16,
    flags: u16,
    size: u32
}

#[repr(C)]
struct MultibootInfoTag {
    header: HeaderTag,
    requests: RequestType,
}

#[repr(C)]
struct MultibootHeader {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    multiboot_info_request: MultibootInfoTag,
    alignment_tag: HeaderTag,
    end_tag: HeaderTag
}


macro_rules! header_checksum {
    () => {
        -((HEADER_MAGIC + HEADER_ARCH + size_of::<MultibootHeader>() as u32) as i32) as u32
    };
}

#[link_section = ".boot.multiboot"]
#[no_mangle]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic:         HEADER_MAGIC,
    architecture:  HEADER_ARCH,
    header_length: size_of::<MultibootHeader>() as u32,
    checksum:      header_checksum!(),
    multiboot_info_request: MultibootInfoTag {
        header: HeaderTag {
            tag: 1,
            flags: 0,
            size: size_of::<MultibootInfoTag>() as u32
        },
        requests: REQUESTS
    },
    alignment_tag:  HeaderTag {
        tag:   6,
        flags: 0,
        size:  8
    },
    end_tag: HeaderTag {
        tag:   0,
        flags: 0,
        size:  8
    }
};

