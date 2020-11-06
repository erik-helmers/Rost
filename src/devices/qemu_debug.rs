use crate::arch::instructions::outb;

const ISA_DEBUG_EXIT_IOBASE: u16 = 0xf4;

/// Stops QEMU with specified exit code
pub fn exit(code: u8) -> ! {
    unsafe {outb(ISA_DEBUG_EXIT_IOBASE, code);}
    panic!("Write to ISA_DEBUG_EXIT port didn't stop QEMU");
}  