/// A macro to import other macros.
/// Nice.
///
/// This imports common macros : such as
///    - serial_print,  serial_println

#[macro_export]
macro_rules! import_commons {
    () => {
        #[allow(unused_import)]
        use rost_nbs::{serial_println, serial_print};
    }
}
