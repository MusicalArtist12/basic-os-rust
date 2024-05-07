use core::{
    ptr::{read_volatile, write_volatile}, 
    fmt::{self, Write}
};
use crate::kernel::sync::mutex::Mutex;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_char: u8,
    attribute: CharAttr
}

// todo: implement mutual exclusion 
pub struct Terminal {
    pub default_attr: CharAttr,
    buffer: *mut ScreenChar,
    width: usize,
    height: usize,
    column_idx: usize
}

impl ScreenChar {
    pub fn new(attr: CharAttr, value: u8) -> ScreenChar {
        ScreenChar {
            ascii_char: value,
            attribute: attr, 
        }
    }
}

impl CharAttr {
    pub const fn new(foreground: Color, background: Color) -> CharAttr {
        CharAttr((background as u8) << 4 | (foreground as u8))
    }
}

impl Terminal {
    fn print_char(&mut self, val: u8) {
        if val == b'\n' {
            self.newline();
        }
        else {
            self.write((self.column_idx, self.height - 1), val, None);
            self.column_idx += 1;
        }
    }

    fn newline(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {
                let character = self.read((col, row));
                self.write((col, row), b' ', None);

                match character {
                    Some(x) => self.write((col, row - 1), x.ascii_char, Some(x.attribute)),
                    None => {}
                }        
            }
        }

        self.column_idx = 0;
    }

    // constructor for vga text mode 80x25
    pub const fn vga_text_mode(def_attr: CharAttr) -> Terminal {
        Terminal {
            default_attr: def_attr,
            buffer: 0xb8000 as *mut ScreenChar,
            width: 80,
            height: 25,
            column_idx: 0
        }   
    }

    pub fn read(&mut self, pos: (usize, usize)) -> Option<ScreenChar> {
        if pos.0 > self.width || pos.1 > self.height {
            return None;
        }

        let index: isize = (pos.0 + (pos.1 * self.width)) as isize;
        let character: ScreenChar = unsafe {
            read_volatile(self.buffer.offset(index)) as ScreenChar
        };
        return Some(character);
    }

    pub fn write(&mut self, pos: (usize, usize), val: u8, overwrite_attr: Option<CharAttr>) {
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

    pub fn clear_screen(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {    
                self.write((col, row), b' ', None);
            }
        }   
    }

}

// public interface: write_str, write_char, write_fmt
impl fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.print_char(c as u8);
        }
        Ok(())
    }
}

pub static STDOUT: Mutex<Terminal> = Mutex::new(Terminal::vga_text_mode(CharAttr::new(Color::Black, Color::White)));

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::io::basic_vga_driver::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate:::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    
    STDOUT.lock().write_fmt(args).unwrap();
}