use crate::kernel::{MemoryArea, MemoryAreaIter};
use super::{PAGE_SIZE, NUM_FRAMES};

mod frame_table_segment;
pub mod frame_table;

pub use frame_table::*;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]

enum FrameState {
    Free = 0x00,
    Occupied = 0x01,
    System = 0x02,
}

#[derive(Copy, Clone, Debug)]
pub struct Frame {
    pub number: usize,
}

