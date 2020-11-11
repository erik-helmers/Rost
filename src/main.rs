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

use common::multiboot2::{MultibootInfo, MemoryMap, TagHeader};

#[no_mangle]
#[cfg(not(test))]
pub unsafe extern "sysv64" fn _start(_boot_info: *const ()) {
    // We have many things to redo now that we're in higher half 
    // - setup a better GDT 
    // - setup better paging
    serial_println!("Trying to set IDT...");

    arch::interrupts::init_idt();
    
    let mbi = MultibootInfo::new(_boot_info) ;
    for (i,tag) in (&mbi).into_iter().enumerate() {
        let tag = tag.as_ref().unwrap();
        serial_println!("Tag #{} : {:?}",i, tag);
    }
    let mmap = mbi.find::<MemoryMap>().unwrap();
    serial_println!("{:?}", mmap);
    for i in 0..mmap.nb_entries(){
        serial_println!("Entry #{}: {:?}", i, mmap[i]);
    }

    devices::qemu_debug::exit(devices::qemu_debug::Status::Success);

}

#[no_mangle]
#[cfg(test)]
pub unsafe extern "sysv64" fn _start(_boot_info: *const u8) -> !{
    use devices::qemu_debug;
    qemu_debug::exit(qemu_debug::Status::Success);
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler(info);
}
