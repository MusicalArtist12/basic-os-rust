use core::ptr;

#[repr(u8)]
enum Color {
    Black   = 0,
    Blue    = 1,
    Green   = 2,
    Cyan    = 3,
    Red     = 4,
    Magenta = 5,
    Brown   = 6,
    LGray   = 7,
    DGray   = 8,
    LBlue   = 9,
    LGreen  = 10,
    LCyan   = 11,
    LRed    = 12,
    Pink    = 13,
    Yellow  = 14,
    White   = 15,
}

#[repr(transparent)]
struct CharAttr(u8);

impl CharAttr {
    fn new(blink: bool, foreground: Color, background: Color)-> CharAttr {
        CharAttr((blink as u8) << 15 | (foreground as u8) << 4 | (background as u8))
    }

    fn value(&mut self) -> u8 {
        (self as *const _) as u8
    }
}

#[repr(C)]
struct ScreenChar {
    character: u8,
    attribute: CharAttr
}

impl ScreenChar {
    fn new(attr: CharAttr, value: u8) -> ScreenChar {
        ScreenChar {
            character: value,
            attribute: attr, 
        }
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

struct VGAInterface {
    column_position: usize,
    default_attr: CharAttr,
    buffer: *mut Buffer
}

impl VGAInterface {
    pub fn set_byte(&mut self, pos: (u8, u8), val: ScreenChar) {

    }
}