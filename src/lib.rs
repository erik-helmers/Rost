#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]
extern crate rlibc;


#[no_mangle]
pub extern fn _start() {
    unsafe {
        let ptr= 0xb8000 as *mut u32;
        for i in 0..100 {
            *(ptr.offset(i)) = 0x07690748;

        }
    }
    loop{}    
}


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        self();
    }
}


pub fn test_runner(tests: &[&dyn Testable]) {
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}

#[test_case]
fn test_lib(){
    assert!(0==0);
}


