
use volatile::Volatile;

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column: 0,
        row: 0,
        style: Style::new(Color::LightGreen, Color::Black, false),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}


// Buffer size 
const BUFFER_HEIGHT : usize = 25;
const BUFFER_WIDTH : usize = 80;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Represents a VGA color 
// Background color shouldn't use value greater than 0x7 
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xa,
    LightCyan = 0xb,
    LightRed = 0xc,
    Pink = 0xd,
    Yellow = 0xe,
    White = 0xf,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Style(u8);


impl Style {
    pub const fn new(fg:Color, bg:Color, blink:bool) -> Self {
        return Style((blink as u8) << 7 | (bg as u8) << 4 | (fg  as u8));
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    char : u8,
    style: Style
}
/* #[repr(transparent)]
pub struct CursorPos(u8);

impl CursorPos {
    pub fn new(row: u8, column: u8) -> Self {
        return CursorPos(row);
    }
} */
#[repr(transparent)]
pub struct Buffer(pub [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]);


pub struct Writer {
    pub buffer: &'static mut Buffer,
    pub column: usize,
    pub row: usize,
    pub style: Style
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
         Ok(())
    }
}
impl Writer {
    
    pub fn write(&mut self, txt: &str) {
        for byte in txt.bytes() {
            //TODO: fix line wrap
            //TODO: line return
            self.write_char(byte);
        }
     }

    fn write_char(&mut self, chr: u8){
        match chr as char {
            '\n'  => self.newline(),
            // Set character and update cursor
            _ => {
                self.buffer.0[self.row][self.column].write(ScreenChar{char:chr, style:self.style});
                self.column += 1;
                if self.column >= BUFFER_WIDTH {
                    self.newline();
                }
                
            }
        }

    }

    pub fn newline(&mut self){
        
        if self.row + 1 >= BUFFER_HEIGHT {
            self.scroll_down(1);
        }

        self.column = 0;
        self.row += 1;
        
    }
    
    pub fn scroll_down(&mut self, line_count: usize){
        // We copy the lines from top to bottom
        for row in 0..BUFFER_HEIGHT-line_count  {
            // Set the current line content to  content from n+line_count one
            for column in 0..BUFFER_WIDTH {
                self.buffer.0[row][column].write(self.buffer.0[row+line_count][column].read());
            }
        }
        // We zero the 'new' lines with the current style
        for row in BUFFER_HEIGHT-line_count..BUFFER_HEIGHT {
            for column in 0..BUFFER_WIDTH {
                self.buffer.0[row][column].write(ScreenChar{char:0, style:self.style});
            }
        }

        // Finally the cursor should go up too
        self.row = if self.row < line_count { 0 } else { self.row-line_count };

    }



}