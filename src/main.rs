
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rost::test_runner)]
#![reexport_test_harness_main = "test_main"]


use rost::{println, print};
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

extern crate alloc;

use alloc::boxed::Box;
use rost::task::{executor::Executor, keyboard::print_keypresses};
use rost::task::Task;

async fn fun() -> u32{
    3
}

async fn some_task(){
    let num =  fun().await;
    println!("Task found code {}", num);
}

fn mem_init(boot_info: &'static BootInfo){
    
    
    use x86_64::{structures::paging::{MapperAllSizes, Page}, VirtAddr};
    use rost::memory;
    use rost::memory::BootInfoFrameAllocator;
    use rost::allocator;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
                .expect("heap alloc failed");
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    
    rost::init();
    println!("Rost is alive !");

    mem_init(boot_info);


    let mut executor = Executor::new();
    executor.spawn(Task::new(some_task()));
    executor.spawn(Task::new(print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("So long...");
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



/// Interrupts should not cause kernel panics
#[test_case]    
fn brk_int3() {
    x86_64::instructions::interrupts::int3();
}



/// Testing if many float calculations works 
/// 
/// Interrupt and FPUs don't work well together
/// (i.e. 512bytes of data has to be saved for context switching)
/// So the kernel should NOT use those and rather use soft floating point
/// see file `x86_64-rost.json:14` 
#[test_case]
fn many_float_works_with_interrupts(){
    let mut acc1 = 0f32;
    let mut acc2 = 0f32;
    let mut acc3 = 0f32;
    for _ in 0..1_000_000 {
        acc1 += 0.5;
        acc2 += 1.5f32;
        acc3 += 3.5;
    }
    assert_eq!(acc1, 500_000f32);
    assert_eq!(acc2, 1_500_000f32);
    assert_eq!(acc3, 3_500_000f32);

}