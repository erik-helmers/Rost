use core::ops::{Add, AddAssign, Sub};
use crate::utils::maths::{align_upper, align_lower};


/// Implements the Address related traits
/// 
/// 
/// lol no blanket impl for private trait 
macro_rules! impl_addr_traits{
    ($type: ident) => {
        impl Add<usize> for $type {
            type Output = Self;
            #[inline]
            fn add(self, rhs: usize) -> Self::Output {
                Self {addr: self.addr + rhs}
            }
        }
        impl Add<usize> for &$type {
            type Output = $type;
            #[inline]
            fn add(self, rhs: usize) -> Self::Output {
                $type{addr: self.addr + rhs}
            }
        }
        impl Add<usize> for &mut $type {
            type Output = $type;
            #[inline]
            fn add(self, rhs: usize) -> Self::Output {
                $type{addr: self.addr + rhs}
            }
        }
        impl AddAssign<usize> for $type{
            #[inline]
            fn add_assign(&mut self, rhs: usize) {
                *self = self.add(rhs);
            }
        }
        
        impl Sub<usize> for $type {
            type Output = Self;
            fn sub(self, rhs: usize) -> Self::Output {
                Self {addr:self.addr - rhs}
            }
        }



        impl $type {
            #[inline]
            pub fn align_lower(&self, pad: usize) -> Self {
                Self {addr: align_lower(self.addr, pad)}
            }
            #[inline]
            pub fn align_upper(&self, pad: usize) -> Self {
                Self {addr: align_upper(self.addr, pad)}
            }    
        }

    }
}






#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysAddr{
    pub addr: usize
}

impl PhysAddr {
    pub fn new(addr: usize) -> Self {
        Self {addr}
    }

    pub const fn null() -> Self {
        Self {addr: 0}    
    }

    #[inline(always)]
    pub fn as_usize(&self) -> usize {
        self.addr 
    }

}

impl_addr_traits!(PhysAddr);


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtAddr{
    pub addr: usize
}


impl_addr_traits!(VirtAddr);

impl VirtAddr {
    /// Creates a new VirtAddr with check 
    pub fn new(addr: usize) -> Self {
        assert!(Self::is_valid(addr), "Incorrect address.");
        Self {addr}
    }

    /// Creates a new VirtAddr without check
    /// 
    /// Safety: 
    /// The addr should be valid sign from bit 47
    pub unsafe fn new_unchecked(addr: usize) -> Self {
        Self {addr}
    }
}


#[cfg(feature="x86_64")]
impl VirtAddr {
    pub fn is_valid(addr: usize) -> bool {
           addr  < 0x0000_8000_0000_0000 
        || addr >= 0xffff_8000_0000_0000
    }
}

impl VirtAddr {

    #[inline]
    pub fn as_ptr<T>(&self) -> *const T {
        self.addr as *const T
    }

    #[inline]
    pub fn as_ptr_mut<T>(&self) -> *mut T {
        self.addr as *mut T
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        self.addr 
    }
}
