use crate::utils::port::*;
use crate::arch::instructions::{inb, outb};
use spin::Mutex;

pub static SERIAL_PRINTER : Mutex<SerialPrinter> = Mutex::new(SerialPrinter::new());



#[macro_export]
macro_rules! serial_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::devices::serial_print::_print(format_args!($($arg)*)));
}


#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_PRINTER.lock().write_fmt(args).unwrap();
}

#[allow(dead_code)]
pub struct SerialPrinter {

    base_port: u16,

    /// When DLAB clear this is the DAT register 
    /// With DLAB set, this is the LSB of divisor value
    p_data: Port<u8>,
    /// With DLAB clear this is the Interrupt enable register
    /// With DLAB set, this is the MSB of divisor value. 
    p_interrupt: Port<u8>,
    /// Interrupt Identification and FIFO control registers 
    p_intr_id: Port<u8>,
    /// Line Control Register. The most significant bit of this register is the DLAB. 
    p_line_ctrl: Port<u8>,
    /// Modem Control Register. 
    p_modem_ctrl: Port<u8>,
    /// Line Status Register. 
    p_line_stat: Port<u8>,
    /// Modem Status Register. 
    p_modem_stat: Port<u8>,
    /// Scratch Register. 
    p_scratch: Port<u8>,
}

#[repr(u8)]
/// Represents a parity type
///
/// The value can just be ored with the p_line_ctrl port
pub enum ParityType {
    NONE  = 0xff & 0b000 << 3,
    ODD   = 0xff & 0b001 << 3,
    EVEN  = 0xff & 0b011 << 3,
    MARK  = 0xff & 0b101 << 3,
    SPACE = 0xff & 0b111 << 3
}


impl core::fmt::Write for SerialPrinter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);
         Ok(())
    }
}

impl SerialPrinter {
    pub const fn new() -> Self {
        let base_port = SerialPrinter::search_com_port();
        Self {
            base_port,
            p_data:       Port::new(base_port + 0),
            p_interrupt:  Port::new(base_port + 1),
            p_intr_id:    Port::new(base_port + 2),
            p_line_ctrl:  Port::new(base_port + 3),
            p_modem_ctrl: Port::new(base_port + 4),
            p_line_stat:  Port::new(base_port + 5),
            p_modem_stat: Port::new(base_port + 6),
            p_scratch:    Port::new(base_port + 7)
        }    
    }

    /// Inits the Serial COM
    ///
    /// I implemented all the other function to initialize step by step 
    /// the COM but honnestly this is much less verbose and easier to read
    pub unsafe fn init(&self) {
        let port = self.base_port;
        outb(port + 1, 0x00);    // Disable all interrupts
        outb(port + 3, 0x80);    // Enable DLAB (set baud rate divisor)
        outb(port + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
        outb(port + 1, 0x00);    //                  (hi byte)
        outb(port + 3, 0x03);    // 8 bits, no parity, one stop bit
        outb(port + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
        outb(port + 4, 0x0B);    // IRQs enabled, RTS/DSR set
    }   

    pub fn read_ready(&self) -> bool {
        return (unsafe {inb(self.base_port + 5)} & 1) != 0;
    }

    /// Waits until a char is available then read it.
    pub fn read(&self) -> char {
        while !self.read_ready(){};
        return unsafe {inb(self.base_port)} as char;
    }

    pub fn write_ready(&self) -> bool{
        return unsafe{inb(self.base_port + 5)} & 0x20 != 0;
    }
    /// Wait until a char is writable then write it.
    pub fn write_char(&self, a: char){
        while !self.write_ready(){};
        unsafe{ outb(self.base_port, a as u8) }
    }
    /// Write every char to serial.
    pub fn write(&self, s: &str){
        for c in s.bytes(){
            self.write_char(c as char);
        }
    }


    /// Search for a valid COM port
    ///
    /// https://wiki.osdev.org/Serial_Ports#Port_Addresses 
    pub const fn search_com_port() -> u16 {
        return 0x3F8;
    }




    
    // This section is pretty much useless 
    // I'm not sure why i coded it

    pub fn set_baud_rate(&self, divisor: u16){
        // Set DLAB 
        self.p_line_ctrl.write(self.p_line_ctrl.read() | 0x80);
        // Send LSB then MSB
        self.p_data.write(divisor as u8);
        self.p_interrupt.write((divisor >> 8) as u8);
        // Clear DLAB
        self.p_line_ctrl.write(self.p_line_ctrl.read() & 0x3F);
    }
    pub fn set_char_length(&self, length: u8){
        let val = 0xfc | match length {
            5 => 0b00,
            6 => 0b01,
            7 => 0b10,
            8 => 0b11,
            _ => panic!("Invalid length")
        };
        self.p_line_ctrl.write(self.p_line_ctrl.read() | val);
    }
    pub fn set_stop_bits(&self, val: u8){
        // Set bit 2 to 0 or 1
        let val = 0xff & (val << 2);
        self.p_line_ctrl.write(self.p_line_ctrl.read() | val);
    }
    pub fn set_port_parity(&self, val: ParityType){
        self.p_line_ctrl.write(self.p_line_ctrl.read() | val as u8);
    }
}




#[test_case]
pub fn serial_char_no_crash(){
    let sp = SerialPrinter::new();
    unsafe { sp.init();}
    sp.write_char('0');
}

#[test_case]
pub fn serial_str_no_crash(){
    let sp = SerialPrinter::new();
    unsafe { sp.init();}
    sp.write(&"123456789");
}
