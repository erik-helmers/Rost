
#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test::runner)]
#![reexport_test_harness_main = "test_main"]


#![feature(asm)]
#![allow(unused_unsafe)]

use rost_nbs::{*, common::memory::{VirtAddr, alloc::{FrameAllocator, IncrAllocator}, paging::{ActivePageTable, Page}}};
use common::memory::{self as mem,*};
use arch::paging::PageDescriptorFlags as PDF;

import_commons!();

entry_point!(main);


pub fn main(mbi: &'static MultibootInfo) {
    // We have many things to redo now that we're in higher half 
    // - setup a better GDT 
    // - setup better paging

    arch::gdt::init_gdt();
    arch::interrupts::init_idt();

    let p4 = VirtAddr::new(0o177_777_776_776_776_776_0000);
    // Safe if the page is recursively mapped
    let mut rapt4 = unsafe {
        mem::paging::ActivePageTable::new(&*p4.as_ptr())
    };


    let mut alloc = IncrAllocator::new(
        mbi.find().unwrap(), mbi.find().unwrap());


    serial_print!("Testing virtual address translation : ");

    let main_virt_addr = VirtAddr::new(main as usize);    
    let main_phys_addr = rapt4.translate(main_virt_addr).expect(
        "Failed to translate.");

    serial_println!("[ok]");
    //serial_println!("Translation from {:?} to {:?} found.", main_virt_addr, main_phys_addr);
    test_huge_pages(&mut rapt4, &mut alloc);



    devices::qemu_debug::exit(devices::qemu_debug::Status::Success);
}

fn test_huge_pages<A>(rapt4: &mut ActivePageTable, alloc: &mut A) 
    where A: FrameAllocator{
    

    serial_print!("Testing huge page allocation : ");

    let addr = VirtAddr::new(0o251_042_123_000_0000); 
    assert!(rapt4.translate(addr).is_none());


    let page = Page::new(addr, SizeType::HugeP2);  
    let frame = alloc.allocate(SizeType::HugeP2);

    /* serial_println!("None = {:?}, map to {:?}",
            rapt4.translate(addr),
            frame);
     */
    rapt4.map_to(page, frame, PDF::PRESENT, alloc);
    assert!(rapt4.translate(addr).is_some());
    assert!(rapt4.translate(addr+0x2000).is_some());
    //serial_println!("Some = {:?}", rapt4.translate(addr));

    //serial_println!("next free frame: {:?}", alloc.allocate(SizeType::Page));
    rapt4.unmap(page, alloc);
    assert!(rapt4.translate(addr).is_none());
    assert!(rapt4.translate(addr+0x2000).is_none());
    //serial_println!("None = {:?}", rapt4.translate(addr));
    serial_println!("[ok]");
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test::panic_handler(info);
}
