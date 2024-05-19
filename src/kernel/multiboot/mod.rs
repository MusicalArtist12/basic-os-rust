// https://www.gnu.org/software/grub/manual/multiboot2/multiboot.pdf

mod multiboot_header;
pub mod multiboot_info;

pub use multiboot_info::MultibootInfo;


use multiboot_info::TagID;

type RequestType = [TagID; 2];
const REQUESTS: RequestType = [TagID::MemInfo, TagID::MemMap];
