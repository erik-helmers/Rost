
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rost::test_runner)]
#![reexport_test_harness_main = "test_main"]


use rost::{println};
use bootloader::BootInfo;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}






#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    rost::init();
    //println!("Hello from Rost!");

    use x86_64::{structures::paging::{MapperAllSizes, Page}, VirtAddr};

    use rost::memory;
    use rost::memory::{BootInfoFrameAllocator};



    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeef000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { 
        page_ptr.offset(400).write_volatile(0xf020_f077_f065_f04e);
        page_ptr.offset(401).write_volatile(0xf06d_f06f_f072_f066);
        page_ptr.offset(402).write_volatile(0xf077_f065_f06e_f020);
        page_ptr.offset(403).write_volatile(0xf067_f061_f070_f020);
        page_ptr.offset(404).write_volatile(0xf020_f021_f020_f065);
    };


    //rost::hlt_loop();

    //memory::explore_page_table(unsafe {memory::active_level_4_table(phys_mem_offset)}, 4, 3,boot_info.physical_memory_offset);

    println!("Physical memory offset: 0x{:016o}", boot_info.physical_memory_offset);
    println!("Physical memory offset: P1: {}, P2: {}, P3: {}, P4: {}",
    u16::from(phys_mem_offset.p1_index()) ,
    u16::from(phys_mem_offset.p2_index()) ,
    u16::from(phys_mem_offset.p3_index()) ,
    u16::from(phys_mem_offset.p4_index()) );

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,

        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt) ;
        println!("{:?} -> {:?}", virt, phys);
    }



    #[cfg(test)]
    test_main();

    println!("It did not crash !");

    // stack_overflow();

    // trigger a page fault
/*     unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };
 */    
    rost::hlt_loop();

}

use core::panic::PanicInfo;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //println!("{}", info);
    println!("{}", info);
    rost::hlt_loop ()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rost::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn brk_int3() {
    x86_64::instructions::interrupts::int3();
}
