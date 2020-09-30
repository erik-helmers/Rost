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

use rost::multitasking::{Task, create_task, Stack, switch_task, init_multitasking};
use alloc::boxed::Box;
use alloc::rc::Rc;

use lazy_static::lazy_static;
use spin::Mutex;




static mut  T1: *mut Task = 0 as *mut Task;
static mut  T2: *mut Task = 0 as *mut Task;


pub fn task_1(){
    loop {
        println!("In Task 1 !");
        switch_task(unsafe {&mut *T1},unsafe {&mut  *T2});    
        }
    }


pub fn task_2(){
    loop {
        println!("In task 2 !");
    
        switch_task(unsafe {&mut *T2},unsafe {&mut  *T1});    

    }
}


use rost::arch::pit::*;
use rost::arch::rtc::{RTC, Register};
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {


    let mut rtc: RTC = RTC::new();

    rost::init();
    println!("Rost is alive !");

    //unsafe { rost::arch::instructions::cli();}

    mem_init(boot_info);

    rtc.init();
    rtc.print_date();
    rtc.print_time();

    let stack  =  Stack::new(4096);
    let stackbase = stack.array.first().expect("") as *const u8 as usize;
    println!("Stack start addr: {:#x}, stacktop: {:#x}", stackbase, stack.top_ptr() as usize);


    let kern_tsk =&mut *unsafe {init_multitasking()};

    // let's init the tasks :
    unsafe {
        let t1 = &mut *create_task(task_1);
        let t2 = &mut *create_task(task_2);
        println!("t1 : {:#x}, t2: {:#x}", &*t1 as *const Task as  usize, &*t2 as *const Task as usize );
        T1 = t1 as *mut Task;
        T2 = t2 as *mut Task;
        println!("T1 : {:#x}, T2: {:#x}", T1 as usize, T2 as usize );
        print!("init jmped to task1: \n\t\t");

        switch_task(kern_tsk,&mut *T1);
    
    

    }     

    #[cfg(test)]
    test_main();

    let mut ch1 = unsafe { Channel::new(0) };
    ch1.set_frequency(OperatingMode::RateGenerator, 20 as f64);
    rost::hlt_loop();
    
    //let mut executor = Executor::new();
    //executor.spawn(Task::new(some_task()));
    //executor.spawn(Task::new(print_keypresses()));
    //executor.run();   
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