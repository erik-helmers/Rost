
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rost::test_runner)]
#![reexport_test_harness_main = "test_main"]


use rost::{interrupts};



// use crate::println;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}




/* 
//#[test_case]
fn scroll_test(){
    println!("Printing 50 lines.");
    for i in 1..=50 {
        println!("This is the test line number {}", i);
        for i in 1..=1000 {
            let a = i+i;
        }
    }
    println!("Test finished.");
}

 */


#[no_mangle]
pub extern "C" fn _start() -> ! {
    rost::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash !");
    
    // stack_overflow();

    // trigger a page fault
/*     unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
 */    
    loop {}
}

use rost::println;
use core::panic::PanicInfo;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //println!("{}", info);
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rost::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn brk_int3() {
    x86_64::instructions::interrupts::int3();
}
