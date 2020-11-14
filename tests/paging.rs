
#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test::runner)]
#![reexport_test_harness_main = "test_main"]


#![feature(asm)]
#![allow(unused_unsafe)]

use rost_nbs::*;
use common::memory::{self as mem,*};

import_commons!();

entry_point!(main);


pub fn main(_mbi: &'static MultibootInfo) {
    // We have many things to redo now that we're in higher half 
    // - setup a better GDT 
    // - setup better paging

    arch::gdt::init_gdt();
    arch::interrupts::init_idt();

    serial_print!("Testing virtual address translation : ");

    let p4 = VirtAddr::new(0o177_777_776_776_776_776_0000);
    // Safe if the page is recursively mapped
    let rapt4 = unsafe {
        mem::paging::ActivePageTable::new(&*p4.as_ptr())
    };

    let main_virt_addr = VirtAddr::new(main as usize);    
    let main_phys_addr = rapt4.translate(main_virt_addr).expect(
        "Failed to translate.");

    serial_println!("[ok]");
    //serial_println!("Translation from {:?} to {:?} found.", main_virt_addr, main_phys_addr);
    

    devices::qemu_debug::exit(devices::qemu_debug::Status::Success);
}



#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler(info);
}
