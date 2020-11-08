//! This file defines the following testing facilities :
//! - test runner
//! - panic handler

crate::import_commons!();

use crate::devices::qemu_debug;


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    // We ran all test, we can now exit
    qemu_debug::exit(qemu_debug::Status::Success);
}




pub fn panic_handler(info: &core::panic::PanicInfo)-> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu_debug::exit(qemu_debug::Status::Success);
}

#[cfg(test)]
#[panic_handler]
/// This is the test specific panic handler
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler(info);
}



#[test_case]
fn trivial_assert() {
    assert_eq!(0, 0);
}