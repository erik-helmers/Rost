use crate::arch::instructions::outb;

const ISA_DEBUG_EXIT_IOBASE: u16 = 0xf4;

#[repr(u8)]
pub enum Status {
    /// Any exit code different from this one will be considered as failed.
    Success = 50,
    Failed  = 51,
}

/// Stops QEMU with specified exit code
pub fn exit_code(code: u8) -> ! {
    unsafe {outb(ISA_DEBUG_EXIT_IOBASE, code);}
    panic!("Write to ISA_DEBUG_EXIT port didn't stop QEMU");
}  

pub fn exit(status: Status) -> ! {
    exit_code(status as u8)
}