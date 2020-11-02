#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test_runner)]
#![reexport_test_harness_main = "test_main"]


// We need this import because it defines an extern crate (rlibc)
// which will be used
#[allow(unused_imports)]
use rost_nbs;

pub static MAGIC: u32 = 0xDEADBEEF;


#[no_mangle]
pub extern fn _start() {
    unsafe {
        let ptr= 0xb8000 as *mut u32;
        for i in 0..100 {
            *(ptr.offset(i)) = MAGIC;
        }
    }
    loop{}    
}



#[cfg(test)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}