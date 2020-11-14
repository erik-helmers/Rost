use core::ops::{Bound::*, RangeBounds};

pub trait BitRange: Sized {

    const SIZE: usize = core::mem::size_of::<Self>();

    /// Bits are shifted
    fn set_bit(&mut self, idx: usize, val: bool);
    /// Bits are shifted
    fn get_bit(&self, idx: usize) -> bool;

    /// Bits are shifted
    /// The bits set are [start; end]
    fn set_bits_range(&mut self, start: usize, end: usize, val: Self);

    /// Bits are shifted
    /// The bits set are [start; end]
    fn get_bits_range(&self, start: usize, end: usize) -> Self;

    /// Bits are shifted
    fn set_bits<T: RangeBounds<usize>>(&mut self, range: T, val: Self);
    /// Bits are shifted
    fn get_bits<T: RangeBounds<usize>>(&self, range: T) -> Self;
}
#[macro_export]
macro_rules! __bitrange_impl {
    ($type: ty) => {
        impl BitRange for $type {
            const SIZE: usize = core::mem::size_of::<Self>();
        
            #[inline]
            fn set_bit(&mut self, idx: usize, val: bool) {
                *self = *self & !(1 << idx) | ((val as Self) << idx)
            }
        
            #[inline]
            fn get_bit(&self, idx: usize) -> bool {
                self & (1 << idx) != 0
            }
        
            #[inline]
            fn set_bits_range(&mut self, start: usize, end: usize, val: Self) {
                *self = *self & !(((1<<(end-start) | (1<<end-start)-1 )) << start) | val << start 
            }
        
            #[inline]
            fn get_bits_range(&self, start: usize, end: usize) -> Self{
                *self >> start & (1<<(end-start) | (1<<end-start)-1)
            }
        
            #[inline]
            fn set_bits<T: RangeBounds<usize>>(&mut self, range: T, val: Self) {
                let start = match range.start_bound() {
                    Unbounded => 0,
                    Included(x) => *x,
                    Excluded(x) => x-1
                };
                let end = match range.end_bound() {
                    Unbounded => core::mem::size_of::<Self>(),
                    Included(x) => *x,
                    Excluded(x) => x-1
                };
                self.set_bits_range(start, end, val);
                
            }
        
            #[inline]
            fn get_bits<T: RangeBounds<usize>>(&self, range: T) -> Self {
                let start = match range.start_bound() {
                    Unbounded => 0,
                    Included(x) => *x,
                    Excluded(x) => x-1
                };
                let end = match range.end_bound() {
                    Unbounded => core::mem::size_of::<Self>(),
                    Included(x) => *x,
                    Excluded(x) => x-1
                };
                
                self.get_bits_range(start, end)
            }
            
        
        }
        
    };
}

crate::__bitrange_impl!(u8);
crate::__bitrange_impl!(u16);
crate::__bitrange_impl!(u32);
crate::__bitrange_impl!(u64);
crate::__bitrange_impl!(usize);


#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test_case]
    fn get_bits_correct(){
        let a = 0b10010000u8;
        assert_eq!(a.get_bit(0), false);
        assert_eq!(a.get_bit(7), true);
        assert_eq_bin!(a.get_bits_range(0,3), 0);
        assert_eq_bin!(a.get_bits_range(4,7), 0b1001);
        
         
        assert_eq_bin!(a.get_bits(0..8), 0b10010000);
        assert_eq_bin!(a.get_bits(0..8), 0b10010000);

        assert_eq_bin!(a.get_bits(0..4), 0);
        assert_eq_bin!(a.get_bits(4..8), 0b1001)
    }

    #[test_case]
    fn set_bits_correct(){
        let mut a = 0u8;
        a.set_bit(0, true);
        a.set_bit(7, true);
        assert_eq!(a, 0b1000_0001);

        let mut b = 0u8;
        b.set_bits_range(0, 3, 0b11);
        b.set_bits_range(6, 7, 0b11);
        assert_eq_bin!(b, 0b1100_0011);

        let mut b = 0u8;
        b.set_bits_range(0, 3, 0b111);
        b.set_bits_range(6, 7, 0b11);
        assert_eq_bin!(b, 0b1100_0111);
    }
}
