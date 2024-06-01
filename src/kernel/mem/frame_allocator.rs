use crate::{gigabytes, kernel::{MemoryArea, MemoryAreaIter}, kilobytes, println};

const PAGE_SIZE: usize = kilobytes!(4);
const NUM_FRAMES: usize = gigabytes!(4) / PAGE_SIZE;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]

enum FrameState {
    Free = 0x00,
    Occupied = 0x01,
    System = 0x02,
}

#[derive(Clone, Copy)]
struct FrameAllocatorSegment {
    // 4 GB per allocator max - uses 1 mb
    pub start_frame: usize,
    num_frames: usize,                  // max = NUM_FRAMES
    frames: [FrameState; NUM_FRAMES],
    num_used_frames: usize
}

impl FrameAllocatorSegment {
    fn new(start_frame: usize, num_frames: usize) -> Self {
        FrameAllocatorSegment {
            start_frame: start_frame,
            num_frames: num_frames,
            frames: [FrameState::Free; NUM_FRAMES],
            num_used_frames: 0
        }
    }

    pub fn handles_frame(&self, frame: usize) -> bool {
        self.start_frame <= frame && frame < self.start_frame + self.num_frames
    }

    pub fn set_frame(&mut self, frame: usize, state: FrameState) {
        assert!(self.handles_frame(frame));
        if self.frames[frame - self.start_frame] == FrameState::Free &&
            state != FrameState::Free {
                self.num_used_frames += 1;
        }

        if self.frames[frame - self.start_frame] != FrameState::Free &&
            state == FrameState::Free {
                self.num_used_frames -= 1;
            }

        self.frames[frame - self.start_frame] = state;
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

}

pub struct FrameAllocator {
    // 64 GB max
    num_segments: usize,
    segments: [Option<FrameAllocatorSegment>; 16]
}

impl FrameAllocator {
    pub fn new(
        multiboot_start: usize, 
        multiboot_end: usize,
        kernel_start: usize,
        kernel_end: usize,
        free_areas: MemoryAreaIter
    ) -> Self {
        let mut allocator = FrameAllocator {
            num_segments: 0,
            segments: [None; 16]
        };

        for area in free_areas {
            // println!("{:?}", area);
            if area.typ == 1 && area.base_addr != 0 {
                allocator.map_area(area);
            }
        }

        let mb_start_frame = multiboot_start / PAGE_SIZE;
        let mb_end_frame = multiboot_end / PAGE_SIZE;
        let k_start_frame = kernel_start / PAGE_SIZE;
        let k_end_frame = kernel_end / PAGE_SIZE;

        println!("mb start: {}", mb_start_frame);
        println!("mb end: {}", mb_end_frame);
        println!("kernel start: {}", k_start_frame);
        println!("kernel end: {}", k_end_frame);

        for i in mb_start_frame..mb_end_frame {
            allocator.set_frame(i, FrameState::System);
        }

        for i in k_start_frame..k_end_frame {
            allocator.set_frame(i, FrameState::System);
        }

        for i in 0..allocator.num_segments {
            match &allocator.segments[i] {
                Some(x) => { println!("{} -> start: {}, size: {}",  i, x.start_frame, x.num_frames); },
                None => { }
            }
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
            
            self.segments[self.num_segments] = Some(FrameAllocatorSegment::new(
                (free_area.base_addr as usize / PAGE_SIZE) + (i as usize * NUM_FRAMES),
                num_frames
            ));
            
            self.num_segments += 1;

            // println!("{}", self.num_segments);

            unmanaged_frames -= num_frames;
        };
    }

    fn set_frame(&mut self, frame: usize, state: FrameState) {
        for i in 0..self.num_segments {
            match &mut self.segments[i] {
                Some(segment) => {
                    if segment.handles_frame(frame) {
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
        
        for i in 0..self.num_segments {
            match self.segments[i] {
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
        for i in 0..self.num_segments {
            match self.segments[i] {
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
        for i in 0..self.num_segments {
            match self.segments[i] {
                Some(segment) => {
                    count += segment.slow_used_space();
                },
                None => {}
            }
        };
        
        count  
    }
}