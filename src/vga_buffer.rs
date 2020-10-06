
use volatile::Volatile;

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;


lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(
        unsafe { &mut *(0xb8000 as *mut Buffer) },
        0,
        0,
        Style::new(Color::Yellow, Color::Black, false)
    )
);
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
    use x86_64::instructions::interrupts;
    
    interrupts::without_interrupts(|| {     
        WRITER.lock().write_fmt(args).unwrap();
    });
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
    /// Use `self.set_column()` to modify this value
    pub column: usize,
    pub row: usize,
    pub flush_nl: bool,
    pub style: Style
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
         Ok(())
    }
}


impl Writer {
    
    pub fn new(buffer: &'static mut Buffer, column: usize, row: usize,
     style: Style) -> Self {
        let writer = Writer {
            buffer,
            column,
            row,
            flush_nl: false,
            style
        };
        #[cfg(feature = "disable_cursor")]
        writer.disable_cursor();

        writer
     }
    fn set_column(&mut self, column: usize){
        self.column = column;

        if self.column >= BUFFER_WIDTH {
            self.newline();
        }
    
    }
    
    /// Writes the string to the VGA buffer
    /// 
    /// Autoscrolls down if space is needed 
    pub fn write(&mut self, txt: &str) {
        for byte in txt.bytes() {
            self.write_char(byte);
        }
     }

    fn write_char(&mut self, chr: u8){

        if self.row >= BUFFER_HEIGHT {
            self.scroll_down(1);
        }

        match chr as char {
            '\n'  => self.newline(),
            '\t' => { self.tab(); self.flush_nl = false},
            // Set character and update cursor
            _ => {
                self.buffer.0[self.row][self.column].write(ScreenChar{char:chr, style:self.style});
                self.set_column(self.column + 1);
                self.flush_nl = false;
            }
        }
    }

    

    
    pub fn tab(&mut self){
        let col = self.column + 4;
        self.set_column( col - col % 4 );
    }

    /// Adds a newline
    /// 
    /// This function doesn't scroll down until a character is written
    /// Or when a newline was already called
    pub fn newline(&mut self){

        // If a newline was already added
        if self.flush_nl && self.row >= BUFFER_HEIGHT{
            self.scroll_down(1);
        }

        self.column = 0;
        self.row += 1;
        self.flush_nl = true;
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

    /// Disables the blinking cursor 
    /// 
    /// for more info 
    /// https://wiki.osdev.org/Text_Mode_Cursor#Without_the_BIOS
    pub fn disable_cursor(&self){
        unsafe {
            // FIXME: change this to be plateform agnostic
            crate::utils::x86_64::instructions::outb(0x3D4, 0x0A);
            crate::utils::x86_64::instructions::outb(0x3D5, 0x20);            
        }
    }
}



 
//#[test_case]


#[test_case]
pub fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";

    interrupts::without_interrupts(|| {
        // We use writter to print while writer is locked (avoid timer interrupt dot spam)
        let mut writer = WRITER.lock();
        // Newline to avoid fail because of dotspam
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.0[1][i].read();
            assert_eq!(char::from(screen_char.char), c);
        }
    });
}

#[test_case]
/// Tests auto text scroll 
pub fn test_println_scroll(){
    use x86_64::instructions::interrupts;
    use core::fmt::Write;

    let s1 = "This message should disappear.";
    let s2 = "This message should be on the second line";

    interrupts::without_interrupts(|| {

        let mut writer = WRITER.lock();

        write!(writer, "{}\n\n{}", s1, s2).expect("writeln failed");
        // 24 newlines after the message should be on the second line
        for _ in 0..24 { writeln!(writer, "").expect("writeline failed"); }
        for (i, c) in s2.chars().enumerate() {
            let screen_char = writer.buffer.0[1][i].read();
            assert_eq!(char::from(screen_char.char), c);
        }
    });
    
}
