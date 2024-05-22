// https://www.gnu.org/software/grub/manual/multiboot2/multiboot.pdf

pub mod multiboot_info;
pub use multiboot_info::*;

mod multiboot_header;

type RequestType = [TagID; 3];
const REQUESTS: RequestType = [TagID::MemInfo, TagID::MemMap, TagID::ELFSymbols];
