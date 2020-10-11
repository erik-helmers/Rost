#![no_std]

#![allow(unused_unsafe)]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(abi_x86_interrupt)]

#![feature(alloc_error_handler)] 
#![feature(wake_trait)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_panic)]
#![feature(new_uninit)]
#![feature(naked_functions)]
#![feature(global_asm)]
#![feature(const_btree_new)]
#![feature(option_expect_none)]


pub mod utils;
pub mod devices;
pub mod memory;
pub mod gdt;
pub mod vga_buffer;
pub mod serial;
pub mod interrupts;
pub mod allocator;
pub mod task;
pub mod arch;
pub mod multitasking;

extern crate rlibc;
extern crate alloc;

pub fn init(){
    gdt::init();    
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();    
}



pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}



#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}




// Testing 
use core::panic::PanicInfo;

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


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

/// Entry point for `cargo test`

use bootloader::BootInfo;
pub fn mem_init(boot_info: &'static BootInfo){
    
    
    use x86_64::{VirtAddr};
    use crate::memory::BootInfoFrameAllocator;


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };


    allocator::init_heap(&mut mapper, &mut frame_allocator)
                .expect("heap alloc failed");
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    mem_init(boot_info);
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
