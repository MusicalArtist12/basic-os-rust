use super::HeaderTag;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MemInfo {
    pub header: HeaderTag,
    mem_lower: u32,
    mem_upper: u32 
}