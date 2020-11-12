//! This is a test to check that IDT loading 
//! works correctly. It just loads a breakpoint 
//! handler in the IDT and exits successfully if 
//! said handler is called.
//! 
//! In case of a triple fault, the exit code is not 50
//! and thus will be interpreted as a fail.

#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test::runner)]
#![reexport_test_harness_main = "test_main"]


#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![allow(unused_unsafe)]
use rost_nbs::*;
use rost_nbs::arch::idt::{Frame, IDT};
import_commons!();


entry_point!(main);
pub fn main(_mbi: &'static MultibootInfo){
    
    // Custom interrupt setup
    unsafe {
        IDT.breakpoint_excpt.set_handler(breakpoint);
        IDT.load();
    }

    serial_print!("Testing that IDT is setup correctly : ");
    // This should not be mapped
    unsafe { 
        asm!("int 3");
    }
    panic!("Memory should not have been mapped.");
}

pub extern "x86-interrupt" fn breakpoint(
    _stack_frame: &mut Frame) 
{
    use devices::qemu_debug::*;
    serial_println!("[ok]");
    exit(Status::Success);
}



#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test::panic_handler(info);
}
