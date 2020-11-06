//! This file defines the following testing facilities :
//! - test runner
//! - panic handler

crate::import_commons!();

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
    crate::devices::qemu_debug::exit(0);
}






#[cfg(test)]
#[panic_handler]
/// This is the test specific panic handler
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    crate::devices::qemu_debug::exit(1);
}



#[test_case]
fn trivial_assert() {
    assert_eq!(0, 0);
}