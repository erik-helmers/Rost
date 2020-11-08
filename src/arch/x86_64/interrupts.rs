
use crate::serial_println;
use super::idt::{Frame, IDT, GateDescriptorType, Entry};



pub fn init_idt() {
    unsafe {
        
        IDT.breakpoint_excpt.set_handler(breakpoint);
        IDT.double_fault_excpt.set_handler(double_fault);
        IDT.load();
    }

}


pub extern "x86-interrupt" fn breakpoint(
    stack_frame: &mut Frame)
{
    serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault(
    stack_frame: &mut Frame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
