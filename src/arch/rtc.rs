/*!
 * In this file we implement RTC (real time clock support)
 * As stated n the osdev.org article, we do not try writting to the mode registers 
 * 
 * TODO: implement "update in progress" support, probably by using a task 
 */


use crate::arch::port::{Port};
use crate::println;



const ENABLE_NMI : u8 = 0x00;
const DISABLE_NMI : u8 = 0x80;


// From https://wiki.osdev.org/CMOS
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Register {
    Seconds = 0x00,     //0–59
    Minutes = 0x02,     //0–59
    Hours = 0x04,       //0–23 in 24-hour mode / 1–12 in 12-hour mode, highest bit set if pm
    Weekday = 0x06,     //1–7, Sunday = 1
    DayOfMonth = 0x07,  //1–31
    Month = 0x08,       //1–12
    Year = 0x09,        //0–99
    Century = 0x32,     //(maybe) 19–20?
    StatusRegisterA = 0x0A, 
    StatusRegisterB = 0x0B
}
pub struct RTC {
    selection_port: Port<u8>,
    rw_port: Port<u8>,
    bin_mode: bool, // Whether the date/time are in binary or BCD mode 
    format_24hours: bool // Whether the date/time is stored in 24 or 12 mode

}

impl RTC {
    /**
     * TODO: please note that you SHOULD call the init() method
     * to set the correct bin_mode and format_24hours value
     */
    pub fn new() -> Self {
        Self {
            selection_port: Port::new(0x70),
            rw_port: Port::new(0x71),
            bin_mode:  false,
            format_24hours: false,
        }

    }

    pub fn init(&mut self){
        let reg_b = self.read_register(Register::StatusRegisterB);
        self.format_24hours = reg_b & 0x2 != 0;
        self.bin_mode = reg_b & 0x4 != 0; 
        println!("RTC mode ({:#x}) : 24hours : {}, bin_mode : {}", reg_b, self.format_24hours, self.bin_mode);
    }


    /**
     * Read from the register and performs conversion if needed 
     * 
     * TODO: Beware, this function always set the NMI flag 
     * however it could be possible to read from the 0x70 port 
     * to keep the NMI disable flag unchanged
     * 
     */
    pub fn read_register(&self, reg: Register) -> u8 {
        let reg = unsafe {
            self.selection_port.write(ENABLE_NMI | (reg as u8));
            self.rw_port.read()
        };

        if self.bin_mode { return reg; }
        else { return RTC::bcd_to_bin(reg)}
    }



    /// Why the fuck would anyone use BCD to represent any number whatsoever ?
    /// 
    /// TODO: this should be in a utils module 
    /// TODO: why would i want to support 12hours mode ? 
    pub fn bcd_to_bin(val: u8) -> u8 {
        (val >> 4) * 10 + (val & 0xf) 
    }

    pub fn print_time(&self)  {
        if !self.format_24hours
         {panic!("Your RTC is in 12pm mode. It isn't supported because it sucks, and i don't want to handle it.");}

        let (hour, min, sec) = (
            self.read_register(Register::Hours),
            self.read_register(Register::Minutes),
            self.read_register(Register::Seconds)
        );
        
        println!("{:02}:{:02}:{:02}", hour, min, sec);
    }


    pub fn print_date(&self){
        let (day, month, year) = (
            RTC::bcd_to_bin(self.read_register(Register::DayOfMonth)),
            RTC::bcd_to_bin(self.read_register(Register::Month)),
            RTC::bcd_to_bin(self.read_register(Register::Year)),
        );
        println!("{}/{}/{}", day, month, year);
    
    }
}



#[test_case]
pub fn time_no_crash(){
    let mut rtc = RTC::new();
    rtc.init();
    rtc.print_time();
}

#[test_case]
pub fn date_no_crash(){
    let mut rtc = RTC::new();
    rtc.init();
    rtc.print_time();
}