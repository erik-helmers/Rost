//! A simpler bitstruct

use paste::paste;

#[macro_export]
macro_rules! bitflags {
    (
        $(#[$meta:meta])*
        $pub:vis struct $name:ident($repr:ty){
            $($rest:tt)*
        }
    ) => {
        $(#[$meta])*
        #[allow(dead_code)]
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
        pub struct $name {
            bits: $repr
        }
        #[allow(dead_code)]
        impl $name {
            $crate::__bitflags_flags!($name; 0; {$($rest)*});
        }

        $crate::__bitflags_impl!($name;$repr);
    };
}

#[macro_export]
macro_rules! __bitflags_flags {
    // flag
    (   $struct: ident; $all: expr;
        {
            $(#[$meta:meta])*
            const $name:ident = $val:expr;
            $($rest:tt)*
        }
    ) => {
        $(#[$meta])*
        pub const $name : Self = Self{bits: $val};

       $crate::__bitflags_flags!(
            $struct; $all | Self::$name.bits; 
            {$($rest)*}
        );
    };
    // end
    (
        $repr:ty;$all:expr; {}
    ) => {
        #[doc(hidden)]
        const __ALL: $repr = Self{bits:$all};
    };
}



#[macro_export]
/// implements the traits for a bitflag struct
/// ive saved something like 40 lines.
macro_rules! __bitflags_impl_trait {
    ($struct: ident, $trait: ident, $func:ident, 
                    $trait_as: ident, $trait_as_fn: ident, 
                    $($eval:tt)*) => 
    {
        impl ::core::ops::$trait for $struct {
            type Output = Self;
            #[inline]
            fn $func(self, r: Self) -> Self::Output {
                Self {bits: { self.bits $($eval)* r.bits }}
            }
        }
        impl ::core::ops::$trait_as for $struct {
            #[inline]
            fn $trait_as_fn(&mut self, rhs: Self) {
                use core::ops::$trait;
                self.bits = self.$func(rhs).bits;
            }
        }

    };
}
#[macro_export]
macro_rules! __bitflags_impl {
    ($name: ident; $repr: ty) => {

        $crate::__bitflags_impl_trait!($name, BitAnd, bitand, BitAndAssign, bitand_assign, & );
        $crate::__bitflags_impl_trait!($name, BitXor, bitxor, BitXorAssign, bitxor_assign, ^ );
        $crate::__bitflags_impl_trait!($name, BitOr, bitor, BitOrAssign, bitor_assign, | );

        impl ::core::ops::Not for $name {
            type Output = Self ;
            
            #[inline]
            fn not(self) -> Self::Output {
                Self{bits: !self.bits}
            }
        }

        impl $name {
            
            #[inline]
            pub const fn empty() -> Self {Self{bits:0}}

            #[inline]
            /// Return value with all flags set
            pub const fn all() -> Self {Self{bits:Self::__ALL.bits}}

            #[inline]
            /// Create from bits, panic if invalid bit set
            pub fn from_bits(val:$repr) -> Self {
                let val = Self{bits:val};
                assert!(Self::all().contains(val));
                val
            }

            #[inline]
            /// Create from bits, truncate all incorrect bits
            pub fn from_bits_truncate(val:$repr) -> Self{
                Self{bits:val} & Self::all()
            }

            #[inline]
            /// Create from bits, leaving potential invalid bits set
            /// Caller must ensure that `val` is correct or use 
            /// safe methods such as `from_bits_truncate`
            pub fn from_bits_unchecked(val:$repr) -> Self {
                Self{bits:val} 
            }

            #[inline]
            pub fn contains(&self, val:Self) -> bool {
                val & !*self == Self::empty()
            }
        }

    }
}


#[cfg(test)]
pub mod test {
    use core::ops::{BitAnd, BitOr, BitOrAssign, BitXor, Not};

    bitflags!(
        struct Flags(u64) {
            const FLAG   = 0b0001;
            const FLAG_2 = 0b0010;
            const FLAG_3 = 0b0100;
        }
    );

    #[test_case]
    fn ops_works(){
        let mut a = Flags::empty();
        a |= Flags::FLAG;
        assert_eq!(a, Flags::FLAG);


    }    

}
