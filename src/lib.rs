#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]
#![feature(global_asm)]
#![feature(const_fn)]
#![allow(unused_macros)]

extern crate rlibc;

pub mod arch;
pub mod utils; 
pub mod devices;
pub mod test; 


#[no_mangle]
#[cfg(test)]
/// This is the library entry point when compiled as a
/// test executable (runs #[test_case] fn's)
pub unsafe extern "sysv64" fn _start(_boot_info: *const u8) {
    test_main();
}
