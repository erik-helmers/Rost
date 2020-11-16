//! A simpler bitstruct

use paste::paste;

#[macro_export]
macro_rules! bitflags {
    (
        $(#[$meta:meta])*
        $pub:vis struct $name:ident($repr:ty){
            $($fields:tt)*
        }
    ) => {
        $(#[$meta])*
        #[allow(dead_code)]
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
        pub struct $name {
            bits: $repr
        }
        #[allow(dead_code)]
        impl $name {
            $crate::__bitflags_flags!($name; 0; {$($fields)*});
        }

        $crate::__bitflags_impl!($name;$repr);
        $crate::__bitflags_impl_debug!($name; $repr; $($fields)*);
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
macro_rules! __bitflags_impl_debug {
    ($name: ident;$repr: ty; $(
        $(#[$_: meta])*
        const $names:ident = $val:expr;
    )*) => {
        
        #[allow(unused_variables, unused_mut, unused_assignments)]
        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut sep = "";
                $(
                    if (self.contains(Self::$names)){
                        f.write_str(sep)?;
                        sep = " | ";
                        f.write_str(::core::stringify!($names))?;
                    }
                )*
                
                if sep == "" { f.write_str("empty")?;}

                Ok(())    
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_fmt(format_args!("{}({})", ::core::stringify!($name), self))
            }
        }
    
    
    }
}




#[macro_export]
/// implements the traits for a bitflag struct
/// ive saved something like 40 lines with this macro.
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
