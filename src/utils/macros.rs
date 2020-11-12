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


/// This macro defines the given function as the kernel entry
/// point. It is called by the boot assembly files in the arch module.
/// 
/// For easier use the MultibootInfo struct is directly imported
/// when using this macro. 
/// 
/// The expected signature of the function is 
/// `fn main(mbi: &'static MultibootInfo) -> ! {}`
/// although it is not checked.
#[macro_export]
macro_rules! entry_point {
    ($name: path) => {
        use $crate::common::multiboot2::MultibootInfo;
        #[no_mangle]
        #[allow(unreachable_code)]
        pub unsafe extern "sysv64" fn _start(_boot_info: *const MultibootInfo) {        
            $name(MultibootInfo::new(_boot_info));
            panic!("Dirty OS exit.");
        }
    }
}

