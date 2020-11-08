
use crate::serial_println;
use super::idt::{Frame, IDT, GateDescriptorType, Entry};



pub fn init_idt() {
    unsafe {

        IDT.breakpoint_excpt.set_handler(breakpoint_handler);
        IDT.load();
    }

}


pub extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut Frame)
{
    serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

