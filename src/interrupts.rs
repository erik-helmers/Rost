use crate::{println, print};
use crate::serial_println;

use crate::gdt;

use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin;


pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;


pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}


lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        // Page fault handler
        idt.page_fault.set_handler_fn(page_fault_handler); 
        
        //Timer handler
        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler); 

        //Keyboard handler
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        
        

        unsafe {
            idt.double_fault
               .set_handler_fn(double_fault_handler)
               .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); 
        }
        idt
    };
}


pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame)
{
    println!("Breakpoint reached \n{:#?}", stack_frame);
}


use x86_64::structures::idt::PageFaultErrorCode;
use crate::hlt_loop;

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("exception: page fault");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    print_isf(stack_frame);
    hlt_loop();
}



// Fast patch because it seems that a panic occurs while printing it
fn print_isf(sf: &InterruptStackFrame)  {
    println!("ExceptionStackFrame {{
    instruction_pointer: 0x{:x?},
    code_segment: {},
    cpu_flags: {:x?},
    stack_pointer: 0x{:x?},
    stack_segment: {}
    }}", sf.instruction_pointer.as_u64(), sf.code_segment, sf.cpu_flags, sf.stack_pointer.as_u64(), sf.stack_segment);

}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    println!("Exception: double fault error : ");
    print_isf(stack_frame);    
    panic!("unable to resume");
}

extern "x86-interrupt" fn timer_interrupt_handler(
    stack_frame: &mut InterruptStackFrame)
{
    print!(".");
    unsafe  {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}


extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: &mut InterruptStackFrame)
{
    use x86_64::instructions::port::Port;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Azerty, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Azerty, ScancodeSet1,
                HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };


    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
