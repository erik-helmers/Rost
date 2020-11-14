use core::{fmt, ops::{Add, AddAssign, Sub}};
use crate::utils::maths::{align_upper, align_lower};
use crate::utils::bitrange::BitRange;
use crate::common::memory::paging::PAGE_SIZE;



/// Implements the Address related traits
/// 
/// 
/// lol no blanket impl for private trait 
macro_rules! impl_addr_traits{
    ($type: ident, $prefix:literal, $name: literal) => {
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
            pub fn is_page_aligned(&self) -> bool {
                &self.align_lower(PAGE_SIZE) == self
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

        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_fmt(format_args!("{}{:#x}", $prefix, self.addr))
            }
        }        
        
        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple($name)
                    .field(&format_args!("{:#x}", self.addr))
                    .finish()
            }
        }


    }
}






#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
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


impl_addr_traits!(PhysAddr, "P", "PhysAddr");


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtAddr{
    pub addr: usize
}


impl_addr_traits!(VirtAddr, "V", "VirtAddr");


impl VirtAddr {
    /// Creates a new VirtAddr with check 
    pub fn new(addr: usize) -> Self {
        Self::try_new(addr).expect("Incorrect address.")
    }

    /// Creates a new VirtAddr without check
    /// 
    /// Safety: 
    /// The addr should be valid sign from bit 47
    pub unsafe fn new_unchecked(addr: usize) -> Self {
        Self {addr}
    }

    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self {addr: ptr as usize}
    }
}


#[cfg(feature="x86_64")]
impl VirtAddr {
    /// Tries to create a valid address
    pub fn try_new(addr: usize) -> Option<Self> {
        match addr.get_bits(47..64) {
            0 | 0x1ffff => Some(VirtAddr{addr}),     
            1 => Some(VirtAddr{addr: addr | (0xffff << 48)}), 
            _ => None,
        }
    }

    /// Creates a new valid virtual address
    /// by dropping any invalid bits
    pub fn new_dropping(mut addr: usize) -> Self {
        let mask = match addr.get_bit(47) {
            false => 0,
            true => 0xffff
        };
        addr.set_bits(48..64, mask);
        Self{addr}
    }


    
    /// Returns the index of the entry
    /// for the page of `level`. For example
    /// ```
    ///     let addr = Virt::new(0o123_456_034_130_5129);
    ///     assert_eq!(addr.table_index(3), 123);
    ///     assert_eq!(addr.table_index(2), 456);
    ///     assert_eq!(addr.table_index(1), 034);
    ///     assert_eq!(addr.table_index(0), 130);
    /// ```
    pub(in crate::common::memory) fn table_index(&self, level: usize) -> usize{
        self.addr >> (12+level*9) & 0o777
    }

    /// Return the offset of the address :
    /// the 12 lowest bits are the offset 
    pub(in crate::common::memory) fn offset(&self) -> usize {
        self.addr & 0xfff
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

