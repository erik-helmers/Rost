#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test::runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]

// We need this import because it defines an extern crate (rlibc)
// which will be used
#![allow(unused_imports)]

use rost_nbs;
use rost_nbs::*;


rost_nbs::import_commons!();


#[no_mangle]
#[cfg(not(test))]
pub unsafe extern "sysv64" fn _start(_boot_info: *const u8) {
    // We have many things to redo now that we're in higher half 
    // - setup a better GDT 
    // - setup better paging
    serial_println!("Trying to set IDT...");

    arch::interrupts::init_idt();

    asm!("int 3");
    //devices::multiboot2::parse_boot_info(_boot_info);
    


    
    
    loop{}
}

#[no_mangle]
#[cfg(test)]
pub unsafe extern "sysv64" fn _start(_boot_info: *const u8) -> !{
    rost_nbs::devices::qemu_debug::exit(0);
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