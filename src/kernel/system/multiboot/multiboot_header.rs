const HEADER_MAGIC: u32 = 0xE85250D6;
const HEADER_ARCH:  u32 = 0;

use super::{REQUESTS, RequestType};

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
    end_tag: HeaderTag
}

macro_rules! info_tag {
    () => {
        MultibootInfoTag {
            header: HeaderTag {
                tag: 1,
                flags: 0,
                size: core::mem::size_of::<MultibootInfoTag>() as u32
            },
            requests: REQUESTS
        }
    }
}

macro_rules! tag_end {
    () => {
        HeaderTag {
            tag:   0,
            flags: 0,
            size:  8
        }
    };
}

macro_rules! sizeof_multiboot_header {
    () => {
        core::mem::size_of::<MultibootHeader>() as u32
    };
}

macro_rules! header_checksum {
    () => {
        -((HEADER_MAGIC + HEADER_ARCH + sizeof_multiboot_header!()) as isize) as u32
    };
}


#[link_section = ".boot.multiboot"]
#[no_mangle]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic:         HEADER_MAGIC,
    architecture:  HEADER_ARCH,
    header_length: sizeof_multiboot_header!(),
    checksum:      header_checksum!(),
    multiboot_info_request: info_tag!(),
    end_tag:       tag_end!()
};

