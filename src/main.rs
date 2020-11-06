#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test_runner)]
#![reexport_test_harness_main = "test_main"]

#![feature(asm)]

// We need this import because it defines an extern crate (rlibc)
// which will be used
#[allow(unused_imports)]
use rost_nbs;
rost_nbs::import_commons!();

use rost_nbs::devices::serial_print::SerialPrinter;

#[no_mangle]
#[cfg(not(test))]
pub unsafe extern "sysv64" fn _start(_boot_info: *const u8) {
    // We have many things to redo now that we're in higher half 
    // - setup a better GDT 
    // - setup better paging 
    let msg = b"Rost is alive."; 
    let color = 0x0f;

    let ptr = 0xb8000;
    for (i,chr) in msg.into_iter().enumerate() {
        *((ptr +(i*2)) as usize as *mut u8 ) = *chr;
        *((ptr +(i*2+1)) as usize as *mut u8 ) = color;
    }
    serial_print!("Here are some numbers : {:#x}\n", (&_boot_info) as *const *const u8 as usize);
    serial_print!("and {:#x}",  0xDEADBEEFu64);
    
    
    loop{}
}

#[no_mangle]
#[cfg(test)]
pub unsafe extern "sysv64" fn _start(_boot_info: *const u8) -> !{
    rost_nbs::devices::qemu_debug::exit(0);
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}