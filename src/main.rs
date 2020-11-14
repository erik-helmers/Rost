#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test::runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]

// We need this import because it defines an extern crate (rlibc)
// which will be used
#![allow(unused_imports)]


use rost_nbs::*;
use arch::paging::PageDescriptorFlags as PDF;
use common::memory::paging::*;
use common::memory::alloc::*;


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

    let mut alloc = IncrAllocator::new(
        mbi.find().unwrap(), mbi.find().unwrap());

    let p4 = VirtAddr::new(0o177_777_776_776_776_776_0000);
    // Safe if the page is recursively mapped
    let mut rapt4 = unsafe {
        ActivePageTable::new(&*p4.as_ptr())
    };


    let addr = VirtAddr::new(42 * 512 * 512 * 4096); // 42th P3 entry
    let page = Page::new(addr);
    let frame = alloc.allocate(4096).expect("no more frames");
    serial_println!("None = {:?}, map to {:?}",
            rapt4.translate(addr),
            frame);
    rapt4.map_to(page, frame, PDF::empty(), &mut alloc);
    serial_println!("Some = {:?}", rapt4.translate(addr));
    serial_println!("next free frame: {:?}", alloc.allocate(4096));

    let _ = unsafe {*(addr.as_ptr::<u8>())};

    rapt4.unmap(page, &mut alloc);


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




