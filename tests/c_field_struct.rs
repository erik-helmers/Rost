#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rost::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(global_asm)]
#![feature(new_uninit)]
extern crate alloc;

use bootloader::{BootInfo};
use core::panic::PanicInfo;


use rost::multitasking;
use alloc::boxed::Box;

global_asm!(include_str!("c_field_struct_asm.s"));




#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    use rost::allocator;
    use rost::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    rost::init();
    rost::mem_init(boot_info);
    
    test_main();

    panic!("Test main should not return");
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rost::test_panic_handler(info)
}



extern "sysv64" {
    fn push_64_pop_8(val:u64) -> u8;
    fn sw_stack_pop(val: u64) -> u64;
}

#[test_case]
fn read_cur_task() {
    
    let mut stack = multitasking::Stack::new(4096);
    /* for val in &[0x01u8,0x23,0x45,0x67,0x89,0xAB,0xCD,0xEF] {
        unsafe { stack.push(*val) }
    }
     */
    const MAGIC_NUMBER: u64 = 0xDEAD_BEEF_15_DEAD;
    unsafe { stack.push_u64(MAGIC_NUMBER)}

    let x = unsafe {sw_stack_pop(stack.top_ptr() as u64)};
    assert_eq!(MAGIC_NUMBER, x);
}



