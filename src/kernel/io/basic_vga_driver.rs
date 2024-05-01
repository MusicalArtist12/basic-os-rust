use core::{char, ptr::{self, read_volatile, write_volatile}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CharAttr(u8);

impl CharAttr {
    pub fn new(foreground: Color, background: Color) -> CharAttr {
        CharAttr((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    attribute: CharAttr
}

impl ScreenChar {
    pub fn new(attr: CharAttr, value: u8) -> ScreenChar {
        ScreenChar {
            ascii_char: value,
            attribute: attr, 
        }
    }
}

pub struct Terminal {
    pub default_attr: CharAttr,
    buffer: *mut ScreenChar,
    width: usize,
    height: usize,
    column_idx: usize
}

impl Terminal {
    pub fn vga_text_mode(def_attr: CharAttr) -> Terminal {
        Terminal {
            default_attr: def_attr,
            buffer: 0xb8000 as *mut ScreenChar,
            width: 80,
            height: 25,
            column_idx: 0
        }   
    }

    pub fn set_char(&mut self, pos: (usize, usize), val: u8, overwrite_attr: Option<CharAttr>) {
        if pos.0 > self.width || pos.1 > self.height {
            return;
        }
        
        let output: ScreenChar = ScreenChar {
            attribute: match overwrite_attr {
                Some(x) => x,
                None => self.default_attr
            },
            ascii_char: val
        };

        let index: isize = (pos.0 + (pos.1 * self.width)) as isize;

        unsafe { 
            write_volatile(self.buffer.offset(index), output);
        }
    }

    fn read_char(&mut self, pos: (usize, usize)) -> Option<ScreenChar> {
        if pos.0 > self.width || pos.1 > self.height {
            return None;
        }

        let index: isize = (pos.0 + (pos.1 * self.width)) as isize;
        let character: ScreenChar = unsafe {
            read_volatile(self.buffer.offset(index)) as ScreenChar
        };
        return Some(character);
    }

    pub fn clear_screen(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {    
                self.set_char((col, row), b' ', None);
            }
        }   
    }

    fn newline(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {
                let character = self.read_char((col, row));
                self.set_char((col, row), b' ', None);

                match character {
                    Some(x) => self.set_char((col, row - 1), x.ascii_char, Some(x.attribute)),
                    None => {}
                }        
            }
        }

        self.column_idx = 0;
    }

    pub fn print_char(&mut self, val: u8, overwrite_attr: Option<CharAttr>) {
        if val == b'\n' {
            self.newline();
        }
        else {
            self.set_char((self.column_idx, self.height - 1), val, overwrite_attr);
            self.column_idx += 1;
        }

    }
}