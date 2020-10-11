//! This is a support for PIC 
//! 
//! https://wiki.osdev.org/PIC#What_does_the_8259_PIC_do.3F

const PIC1_ADDR: u16 = 0x20;

const PIC2_ADDR: u16 = 0x20;

const PIC_EOI: u8 = 0x20;

const ICW1_ICW4: u8 = 0x01;
/// Single mode
const ICW1_SINGLE: u8 = 0x02;
/// Single (cascade) mode
const ICW1_INIT:u8 = 0x10; 
/// 8086/88 (MCS-80/85) mode 
const ICW4_8086: u8 = 0x01;		

use crate::utils::x86_64::instructions;
use instructions::{inb, outb};
struct PIC {
    cmd_port: u16,
    dat_port: u16,
    offset: u8
}
impl PIC {

    fn new(port: u16, offset: u8) -> Self { 
        Self {
            cmd_port: port,  
            dat_port: port+1,
            offset
        }
    }

    unsafe fn init(&self, master:bool) {
        

        let masks = inb(self.dat_port);

        // Start init
        outb( self.cmd_port, ICW1_INIT | ICW1_ICW4);
        outb( self.dat_port, self.offset);
        // tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
        // or tell Slave PIC its cascade identity (0000 0010)
        outb( self.dat_port, if master {4} else {2});

        outb( self.dat_port, ICW4_8086);
        outb( self.dat_port, masks);

    }


    unsafe fn end_of_interrupt(&self){
        outb(self.cmd_port, PIC_EOI);
    }

    unsafe fn disable(&self){
        outb( self.dat_port, 0xff);
    }
}

pub struct ChainedPics(PIC, PIC);

impl ChainedPics {

    pub fn new(offset1: u8, offset2: u8) -> Self {
        Self(PIC::new(PIC1_ADDR, offset1), PIC::new(PIC2_ADDR, offset2))
    }
    pub unsafe fn init(&self){    
        self.0.init(true);
        self.1.init(false);       
    }

    pub unsafe fn disable(&self){
        self.0.disable();
        self.1.disable();
    }

    pub unsafe fn end_of_interrupt(&self, irq: u8){
        if irq >= 8 {
            self.1.end_of_interrupt();
        }
        self.0.end_of_interrupt();
    }
    

}
