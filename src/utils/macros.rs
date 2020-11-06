/// A macro to import other macros.
/// Nice.
///
/// This imports the following common macros : 
///    - serial_print,  serial_println

#[macro_export]
macro_rules! import_commons {
    () => {
        #[allow(unused_import)]
        use $crate::{serial_println, serial_print};
    }
}
