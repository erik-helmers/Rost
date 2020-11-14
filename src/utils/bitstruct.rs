
use paste::paste;

/// This is a WIP
/// Here are some things I kinda want to add
///    - add casting with Value<MyType>(..)
///    - more than 128 bits repr although
///      i'm not sure of the implementation for now
/// 
/// Each field can be either a Flag with a bit offset
/// or a Val with a range specifier 
/// ```
/// bitstruct! {
///     pub struct Flags(u8){
///         const Accessed  : Flag(0);
///         const Readable  : Flag(1);
///         const Code      : Flag(2)
///         const __reserved: Val(3..5);
///         const DPL       : Val(5..=6);
///         const Present   : Flag(7);
///         const Other     : Val(8..+2);
///     }
/// }
/// ```
/// 
#[macro_export]
macro_rules! bitstruct {
    (
        $(#[$outer:meta])*
        $pub: vis struct $name: ident ($repr: ty){
            $(  $(#[$inner:meta])*
                $fpub:vis $fn:ident:
                // This unpacks the field type
                // This is a field type (Val or Flag)
                // end a bitoffset Flag(3), inclusive range Val(3..=5)
                // or exclusive range Val(6..8)
                $type: ident
                ($start:literal
                    $(..$end:literal)?
                    $(..=$end2:literal)?
                    $(..+$end3:literal)?);
            )*
        }
    ) => {
        #[allow(dead_code)]
        $(#[$outer])*
        #[repr(transparent)]
        $pub struct $name{bits: $repr}

        // Impl the common binary operators
        $crate::__bitstruct_operator_impl!($name; $repr);

        // For each field, we implement the related function
        // If the range is inclusive Flag(3..=5) we treat it as an
        // exclusive range Val(3..5+1) 
        $(
            $crate::__bitstruct_field_impl!($name; $repr; $($inner)*;
            $fpub $fn: $type($start$(;$end)?
                             $(;$end2+1)?
                             $(;$end3+$start)?));
        )*

    };
}

#[macro_export]
macro_rules! __bitstruct_operator_impl {
    ($name: ident; $repr: ty) => {
        impl ::core::ops::BitAnd<$name> for $name {
            type Output = Self;
            fn bitand(self, other: Self) -> Self{
                Self{bits: self.bits & other.bits}
            }
        }
        impl ::core::ops::BitAnd<&$name> for &$name {
            type Output = $name;
            fn bitand(self, other: &$name) -> $name{
                $name{bits: self.bits & other.bits}
            }
        }
        impl ::core::ops::BitAnd<&$name> for &mut $name {
            type Output = $name;
            fn bitand(self, other: &$name) -> $name{
                $name{bits: self.bits & other.bits}
            }
        }
        impl ::core::ops::BitAndAssign<&$name> for $name {
            fn bitand_assign(&mut self, other: &$name){
                *self = (&*self & other);
            }
        }

        impl ::core::ops::BitOr<$name> for $name {
            type Output = Self;
            fn bitor(self, other: $name) -> Self{
                Self{bits: self.bits | other.bits}
            }
        }
        impl ::core::ops::BitOr<&$name> for &$name {
            type Output = $name;
            fn bitor(self, other: &$name) -> $name{
                $name{bits: self.bits | other.bits}
            }
        }
        impl ::core::ops::BitOr<&$name> for &mut $name {
            type Output = $name;
            fn bitor(self, other: &$name) -> $name{
                $name{bits: self.bits | other.bits}
            }
        }
        impl ::core::ops::BitOrAssign<&$name> for $name {
            fn bitor_assign(&mut self, other: &$name){
                *self = (&*self | other);
            }
        }

        
    }
}   

#[macro_export]
macro_rules! __bitstruct_field_impl {
    // Implementing the flag
    (
        $struct_name: ident; $repr:ty; $($fmeta:meta)*; $pub:vis $name: ident: Flag($offset: expr)
    ) => {

        #[allow(non_snake_case)]
        #[allow(dead_code)]
        impl $struct_name {
            ::paste::paste!( const [<$name:upper>]: Self = Self{bits: 1<<$offset}; );
            #[inline]
            $(#[$fmeta])*
            /// Returns whether this flag's bit is set or not
            $pub fn $name(&self) -> bool {
                self.bits & (1<<$offset) != 0
            }
            ::paste::paste!{ 
            #[inline]
            $(#[$fmeta])*
            /// Sets the flag bit to `val`
            $pub fn [<set_ $name>](&mut self, val: bool){
                self.bits = (self.bits & !(1 << $offset)) | (val as $repr) << $offset ;
            }}
            
        }
        
    };
    (
        $struct_name: ident; $repr:ty; $($fmeta:meta)*; $pub:vis $name: ident: Val($start:expr;$end:expr)
    ) => {
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        impl $struct_name {
            #[inline]
            $(#[$fmeta])*
            /// Returns this field's value
            $pub fn $name(&self) -> $repr {
                use $crate::utils::bitrange::BitRange;
                self.bits.get_bits_range($start, $end-1)
            }

            ::paste::paste!{ 
            #[inline] 
            $pub fn [<set_ $name>](&mut self, val: $repr){
                let mask = $repr::MAX;
                assert!(val <= mask, "Val ({}) is greater than max value ({})", val, mask);
                self.[<set_ $name _unchecked>](val)
            }}
            
            ::paste::paste!{ 
            #[inline] 
            $pub fn [<set_ $name _unchecked>](&mut self, val: $repr){
                use $crate::utils::bitrange::BitRange;
                self.bits.set_bits_range($start, $end-1, val);
            }}
    
        }
    };
}




mod tests {
    use super::*;
    crate::import_commons!();

    bitstruct! {
        #[derive(Debug)]
        #[allow(non_snake_case)]
        /// This is bitstruct, internally represented as an u8
        pub struct Flags(u8){
            /// This is a flag with offset 0 in the u8
            accessed  : Flag(0);
            readable  : Flag(1);
            code      : Flag(2);
            /// This is a value which spends bit [3;5[
            __reserved  : Val(3..5);
            /// This is a value which spends bit [5;6]
            DPL       : Val(5..=6);
            present   : Flag(7);
        }
    }

    #[test_case]
    pub fn get_flag_valid(){
        let x = Flags{bits: 1};
        assert!(x.accessed());
        assert!(!x.readable());
        assert!(!x.code());
    }

    #[test_case] 
    pub fn set_flag_valid() {
        let mut x = Flags{bits:0};
        x.set_accessed(true);
        assert!(x.accessed());
        x.set_present(true);
        assert!(x.present());
    }

    #[test_case]
    pub fn get_val_valid(){
        let x = Flags{bits: 0b0110_0000};
        assert_eq!(x.DPL(), 3);
    }


    #[test_case]
    pub fn get_val_valid(){
        let mut x = Flags{bits: 0};
        x.set_DPL(3);
        assert_eq!(x.DPL(), 3);
    }
}
