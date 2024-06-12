use super::*;

#[derive(Clone, Copy)]
pub struct FrameTableSegment {
    // 4 GB per allocator max - uses 1 mb
    pub start_frame: usize,
    pub num_frames: usize,                  // max = NUM_FRAMES
    frames: [FrameState; NUM_FRAMES],
    num_used_frames: usize,
    next_frame: usize,
    pub number: usize
}

impl FrameTableSegment {
    pub fn handles_frame(&self, frame: Frame) -> bool {
        self.start_frame <= frame.number && frame.number < self.start_frame + self.num_frames
    }

    pub fn new(start_frame: usize, num_frames: usize, number: usize) -> Self {
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
                    number: i + self.start_frame,
                });
            }
        }; 

        None 
    }

    pub fn deallocate_frame(&mut self, frame: Frame) {
        assert!(self.handles_frame(frame));

        self.frames[frame.number - self.start_frame] = FrameState::Free;
        if frame.number <= self.next_frame {
            self.next_frame = frame.number;
        }
    }

    pub fn is_full(&self) -> bool {
        self.num_used_frames == self.num_frames
    }

}