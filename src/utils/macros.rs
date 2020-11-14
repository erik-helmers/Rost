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



/// Asserts that two expression are equal to each other (using PartialEq).
///
/// On panic, this macro will print the values representation formatten with the given
/// $fmt literal. For example, to get binary repr
/// ```
///     assert_eq_fmt!("{:b}", 0, 1);
///     assert_eq_fmt!("{:b}", 0, 1, "Something went wrong");
/// ```
/// However if additional args are passed a "{}" must be added:
/// ```
///     assert_eq_fmt(0, 1, "{:#x} != {:#x} {}", "Something went wrong")
/// ```
/// 
/// For more info see assert_eq!
#[macro_export]
macro_rules! assert_eq_fmt {
    ($fmt:literal, $a:expr, $b:expr) => { $crate::assert_eq_fmt!($fmt, $a,$b, "")};
    ($fmt:literal, $a:expr, $b:expr, $($rest: tt)*) => {
        {
            let __a = $a;
            let __b = $b;
            ::paste::paste!(
                assert_eq!(__a, __b, 
                    "{} != {} {}", 
                    format_args!($fmt, __a), format_args!($fmt,__b),
                    format_args!($($rest)*));
            );
        }
    }
}

/// Asserts that two expression are equal to each other (using PartialEq).
///
/// On panic, this macro will print their binary representation (using fmt::Bin)
/// 
/// For more info see assert_eq!
#[macro_export]
macro_rules! assert_eq_bin {
    ($($rest:tt)*) => { $crate::assert_eq_fmt!("{:#b}", $($rest)*); };
}


/// Asserts that two expression are equal to each other (using PartialEq).
///
/// On panic, this macro will print their hex representation (using fmt::Hex)
/// 
/// For more info see assert_eq!
#[macro_export]
macro_rules! assert_eq_hex {
    ($($rest:tt)*) => { $crate::assert_eq_fmt!("{:#x}", $($rest)*); };
}
