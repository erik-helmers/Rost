#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]
#![feature(global_asm)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]

#![allow(unused_imports)]
#![allow(unused_macros)]

pub mod arch;
pub mod utils; 
pub mod devices;
pub mod test; 
pub mod common;

pub fn panic_handler(info: &core::panic::PanicInfo) ->! {
    serial_print!("{}", info);
    loop{}
}



#[cfg(test)]
entry_point!(main);

#[cfg(test)]
/// This is the library entry point when compiled as a
/// test executable (runs #[test_case] fn's)
pub fn main(_mbi: &'static MultibootInfo) {
    test_main();
}
