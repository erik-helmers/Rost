//! This is a support for PIC 
//! 
//! https://wiki.osdev.org/PIC#What_does_the_8259_PIC_do.3F

const PIC1_ADDR: u16 = 0x20;
const PIC2_ADDR: u16 = 0xA0;

const PIC_EOI: u8 = 0x20;
const ICW1_ICW4: u8 = 0x01;

/// Single mode
const ICW1_SINGLE: u8 = 0x02;
/// Single (cascade) mode
const ICW1_INIT:u8 = 0x10; 
/// 8086/88 (MCS-80/85) mode 
const ICW4_8086: u8 = 0x01;		

const CMD_READ_ISR: u8 = 0x0a;
const CMD_READ_IRR: u8 = 0x0a;


use crate::utils::x86_64::instructions;
use instructions::{inb, outb, io_wait};
struct PIC {
    cmd_port: u16,
    dat_port: u16,
    offset: u8
}
impl PIC {

    const fn new(port: u16, offset: u8) -> Self { 
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
        io_wait();

        outb( self.dat_port, self.offset);
        io_wait();
        // tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
        // or tell Slave PIC its cascade identity (0000 0010)
        outb( self.dat_port, if master {4} else {2});
        io_wait();

        outb( self.dat_port, ICW4_8086);
        io_wait();

        outb( self.dat_port, masks);
        io_wait();

    }

    /// Reads the ISR from the pic 
    fn read_isr(&self) -> u8 {
        unsafe {
            outb(self.cmd_port, CMD_READ_ISR);
            inb(self.cmd_port)
        }
    }
    fn read_irr(&self) -> u8 {
        unsafe {
            outb(self.cmd_port, CMD_READ_IRR);
            inb(self.cmd_port)
        }
    }

    #[inline]
    /// Returns whether the IRQ on current line
    /// was real or not : i.e. whether it is set 
    /// in the ISR 
    fn spurrious_irq(&self, num: u8) -> bool {
        self.read_isr() & (1 << num) == 0
    }

    #[inline]
    /// Send an EOI (end of interrupt) byte to the PIC
    /// 
    /// SAFETY: it is an error to send an EOI when there
    /// was no event, in particular with spurrious interrupt, so beware
    unsafe fn end_of_interrupt(&self){
        outb(self.cmd_port, PIC_EOI);
    }

    unsafe fn disable(&self){
        outb( self.dat_port, 0xff);
    }
}

pub struct ChainedPics(PIC, PIC);

impl ChainedPics {

    pub const fn new(offset1: u8, offset2: u8) -> Self {
        Self(
            PIC::new(PIC1_ADDR, offset1),
            PIC::new(PIC2_ADDR, offset2)
        )
    }

    pub unsafe fn initialize(&self){    
        self.0.init(true);
        self.1.init(false);       
    }

    pub unsafe fn disable(&self){
        self.0.disable();
        self.1.disable();
    }

    #[inline]
    /// Sends an EOI depending on the IRQ line
    /// Using this 
    pub unsafe fn notify_end_of_interrupt(&self, irq: u8){
        if irq >= 8 {
            self.1.end_of_interrupt();
        }
        self.0.end_of_interrupt();
    }
    
    #[inline]
    /// Should be called before handling IRQ 7/15
    /// If the function returns true, the interrupt routine
    /// should be immediately stopped with no further action
    /// Otherwise it may continue normally (and call EOI at
    /// the end of the routine) 
    pub unsafe fn is_spurrious_interrupt(&self, irq:u8) -> bool {
        
        // Whether the master's interrupt was spurrious
        if irq==7 { return self.0.spurrious_irq(irq)  }
        
        // Whether the slave's interrupt was spurrious
        else if irq==15 {
            // If it was, we send EOI now to the master
            if self.1.spurrious_irq(7) {
                self.0.end_of_interrupt();
                return true;
            }
            return false;
        }

        panic!("Should not be called on IRQ != 7 or 15")
    }

}
