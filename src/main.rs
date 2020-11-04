#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test_runner)]
#![reexport_test_harness_main = "test_main"]


// We need this import because it defines an extern crate (rlibc)
// which will be used
#[allow(unused_imports)]
use rost_nbs;



#[no_mangle]
pub extern fn _start() {
    let msg = b"Rost is alive.";
    let color = 0x0f;
    unsafe {
        let ptr = 0xb8000;
        for (i,chr) in msg.into_iter().enumerate() {
            *((ptr +(i*2)) as usize as *mut u8 ) = *chr;
            *((ptr +(i*2+1)) as usize as *mut u8 ) = color;
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