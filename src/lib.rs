#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]
#![feature(global_asm)]

extern crate rlibc;

pub mod arch;
pub mod utils; 

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


pub fn test_runner(_tests: &[&dyn Testable]) {
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}

#[test_case]
fn test_lib(){
    assert!(0==0);
}


