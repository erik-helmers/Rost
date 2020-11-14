#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test::runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]

// We need this import because it defines an extern crate (rlibc)
// which will be used
#![allow(unused_imports)]

use rost_nbs::{self, common::memory::{self as mem, VirtAddr}};
use rost_nbs::*;


rost_nbs::import_commons!();

use common::multiboot2::{MemoryMap, TagHeader};

#[cfg(not(test))]
entry_point!(main);
#[cfg(test)]
entry_point!(test::main);

pub fn main(mbi: &'static MultibootInfo) {
    // We have many things to redo now that we're in higher half 
    // - setup a better GDT 
    // - setup better paging
    serial_println!("Rost is alive!");

    arch::gdt::init_gdt();
    arch::interrupts::init_idt();


    devices::qemu_debug::exit(devices::qemu_debug::Status::Success);
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler(info);
}



#[cfg(test)]
mod test {
    use super::*;

    pub fn main(_mbi: &MultibootInfo){
        use devices::qemu_debug;
        qemu_debug::exit(qemu_debug::Status::Success);
    }
    
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop{}
    }    
}




