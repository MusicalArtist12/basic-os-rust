use crate::kernel::{MemoryArea, MemoryAreaIter};
use super::{PAGE_SIZE, NUM_FRAMES};
use core::ops::{Index, IndexMut};

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
    pub frame_table_segment: usize

}

#[derive(Clone, Copy)]
struct FrameTableSegment {
    // 4 GB per allocator max - uses 1 mb
    pub start_frame: usize,
    num_frames: usize,                  // max = NUM_FRAMES
    frames: [FrameState; NUM_FRAMES],
    num_used_frames: usize,
    next_frame: usize,
    number: usize
}

impl FrameTableSegment {
    pub fn handles_frame(&self, frame: Frame) -> bool {
        frame.frame_table_segment == self.number && frame.number < self.num_frames
    }

    fn new(start_frame: usize, num_frames: usize, number: usize) -> Self {
        FrameTableSegment {
            start_frame: start_frame,
            num_frames: num_frames,
            frames: [FrameState::Free; NUM_FRAMES],
            num_used_frames: 0,
            next_frame: 0,
            number: number
        }
    }

    pub fn set_frame(&mut self, frame: Frame, state: FrameState) {
        if self.frames[frame.number] == FrameState::Free &&
            state != FrameState::Free {
                self.num_used_frames += 1;
        }

        if self.frames[frame.number] != FrameState::Free && state == FrameState::Free {
            self.num_used_frames -= 1;
        }

        self.frames[frame.number] = state;
    }

    pub fn slow_used_space(&self) -> usize {
        let mut count: usize = 0;
        for i in self.frames {
            if i != FrameState::Free {
                count += 1;
            }
        };
        count * PAGE_SIZE
    }

    pub fn used_space(&self) -> usize {
        self.num_used_frames * PAGE_SIZE
    }

    pub fn allocate_frame(&mut self) -> Option<Frame> {
        if self.is_full() {
            return None;
        }
        
        for i in self.next_frame..self.num_frames {
            if self.frames[i] == FrameState::Free {
                self.next_frame = i + 1;
                self.num_used_frames += 1;
                self.frames[i] = FrameState::Occupied;

                return Some(Frame { 
                    number: i,
                    frame_table_segment: self.number
                });
            }
        }; 

        None 
    }

    pub fn deallocate_frame(&mut self, frame: Frame) {
        assert!(self.handles_frame(frame));

        self.frames[frame.number] = FrameState::Free;
        if frame.number <= self.next_frame {
            self.next_frame = frame.number;
        }
    }

    pub fn is_full(&self) -> bool {
        self.num_used_frames == self.num_frames
    }

}

pub struct FrameTable {
    num_segments: usize,
    segments: [Option<FrameTableSegment>; 64]
}

impl FrameTable {
    pub fn new(
        multiboot_start: usize, 
        multiboot_end: usize,
        kernel_start: usize,
        kernel_end: usize,
        free_areas: MemoryAreaIter
    ) -> Self {
        let mut allocator = FrameTable {
            num_segments: 0,
            segments: [None; 64]
        };

        for area in free_areas {
            if area.typ == 1 && area.base_addr != 0 {
                allocator.map_area(area);
            }
        }

        let mb_start_frame = multiboot_start / PAGE_SIZE;
        let mb_end_frame = multiboot_end / PAGE_SIZE;
        let k_start_frame = kernel_start / PAGE_SIZE;
        let k_end_frame = kernel_end / PAGE_SIZE;

        for i in mb_start_frame..mb_end_frame {
            allocator.set_frame(i, FrameState::System);
        }

        for i in k_start_frame..k_end_frame {
            allocator.set_frame(i, FrameState::System);
        }

        allocator
    }

    fn map_area(&mut self, free_area: MemoryArea) {
        fn round_up(x: usize, y: usize) -> usize {
            (x / y) + (x % y > 0) as usize
        }

        let mut unmanaged_frames = free_area.length as usize / PAGE_SIZE;
        let num_segments = round_up(unmanaged_frames, NUM_FRAMES);
        
        for i in 0..num_segments {
            let num_frames = {
                if unmanaged_frames > NUM_FRAMES {
                    NUM_FRAMES
                } 
                else {
                    unmanaged_frames
                }
            };

            self.segments[self.num_segments] = Some(FrameTableSegment::new(
                (free_area.base_addr as usize / PAGE_SIZE) + (i as usize * NUM_FRAMES),
                num_frames,
                self.num_segments
            ));
            
            self.num_segments += 1;
            unmanaged_frames -= num_frames;
        };
    }

    fn set_frame(&mut self, frame: usize, state: FrameState) {
        for i in self.segments.as_mut() {
            match i {
                Some(segment) => {
                    if frame <= segment.start_frame {
                        continue;
                    }
                    
                    let frame = Frame {
                        frame_table_segment: segment.number,
                        number: frame - segment.start_frame
                    };
                    
                    if segment.handles_frame(frame)  {
                        segment.set_frame(frame, state);
                    }
                },
                None => {
                    return;
                }
            }
        }
    }

    pub fn total_memory(&self) -> usize {
        let mut count: usize = 0;
        for &i in self.segments.as_ref() {
            match i {
                Some(segment) => {
                    count += segment.num_frames * PAGE_SIZE;
                },
                None => {}
            }
        };
        count
    }

    pub fn used_space(&self) -> usize {
        let mut count: usize = 0;
        for &i in self.segments.as_ref() {
            match i {
                Some(segment) => {
                    count += segment.used_space();
                },
                None => {}
            }
        };
        count
    }

    pub fn slow_used_space(&self) -> usize {
        let mut count: usize = 0;
        for i in self.segments.as_ref() {
            match i {
                Some(segment) => {
                    count += segment.slow_used_space();
                },
                None => {}
            }
        };
        count  
    }

    pub fn allocate_frame(&mut self) -> Option<Frame> {
        for i in self.segments.as_mut() {
            match i {
                Some(segment) => {
                    if !segment.is_full() {
                        return segment.allocate_frame()
                    }
                },
                None => {
                    return None;
                }
            }
        }
        None
    }

    pub fn deallocate_frame(&mut self, frame: Frame) {
        for i in self.segments.as_mut() {
            match i {
                Some(segment) => {
                    if segment.handles_frame(frame) {
                        segment.deallocate_frame(frame);
                        break;
                    }
                },
                None => {}
            }
        }
    }
}
