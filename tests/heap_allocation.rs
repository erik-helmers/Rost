#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rost::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;



#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    use rost::allocator;
    use rost::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    rost::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper= unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
            .expect("heap alloc failed");
    test_main();

    panic!("Execution continued after stack overflow");
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rost::test_panic_handler(info)
}

use alloc::boxed::Box;
#[test_case]
fn simple_alloc(){
    let h1 = Box::new(41);
    let h2 = Box::new(42);
    assert_eq!(*h1, 41);
    assert_eq!(*h2, 42);
}


use alloc::vec::Vec;

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}


#[test_case]
fn many_boxes() {
    for i in 0..rost::allocator::HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
