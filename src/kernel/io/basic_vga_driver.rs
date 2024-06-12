const VGA_BUFFER_ADDR: usize = 0x0b8000;
const TAB_LEN: usize = 4;

use core::fmt::{self, Write};

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
    pub const fn new(foreground: Color, background: Color) -> CharAttr {
        CharAttr((background as u8) << 4 | (foreground as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
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
    default_attr: CharAttr,
    buffer_raw: *mut [[ScreenChar; 80]; 25],
    width: usize,
    height: usize,
    column_idx: usize
}


impl Terminal {
    fn buffer(&self) -> &mut [[ScreenChar; 80]; 25] {
        unsafe {&mut *self.buffer_raw}
    }

    fn print_char(&mut self, val: u8) {
        if val == b'\n' {
            self.newline();
        }
        else if val == b'\t' {
            self.print_tab();
        }
        else {
            if self.column_idx >= self.width {
                self.newline();
            }
            self.buffer()[self.height - 1][self.column_idx] = ScreenChar::new(self.default_attr, val);
            self.column_idx += 1;
        }
    }

    fn newline(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {
                self.buffer()[row - 1][col] = self.buffer()[row][col];
                self.buffer()[row][col] = ScreenChar::new(self.default_attr, b' '); 
            }
        }
        self.column_idx = 0;
    }

    fn print_tab(&mut self) {
        for _ in 0..TAB_LEN {
            self.write_char(' ').unwrap();
        }
    }
    
    pub const fn vga_text_mode(def_attr: CharAttr) -> Terminal {
        Terminal {
            default_attr: def_attr,
            buffer_raw: VGA_BUFFER_ADDR as *mut [[ScreenChar; 80]; 25] ,
            width: 80,
            height: 25,
            column_idx: 0
        }   
    }

    pub fn clear_screen(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {    
                self.buffer()[row][col] = ScreenChar::new(self.default_attr, b' ');
            }
        }   
    }

    pub fn change_color(&mut self, def_attr: CharAttr) {
        self.default_attr = def_attr;
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

