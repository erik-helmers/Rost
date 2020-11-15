use core::fmt;

use crate::*;

pub mod paging;
pub mod utils;
pub mod alloc;

pub use utils::{VirtAddr, PhysAddr};

#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SizeType {
    Page = 1,
    HugeP2 = 2,
    HugeP3 = 3
}
impl SizeType {
    /// Returns the corresponding size
    /// To the address
    pub fn size(&self) -> usize {
        match self {
            Self::Page => 4096,
            Self::HugeP2 => 4096*512,
            Self::HugeP3 => 4096*512*512,
        }
    }
    /// Returns the correspondig level to
    /// this size type 
    pub fn level(&self) -> u8 {
        match self {
            Self::Page => 0,
            Self::HugeP2 => 1,
            Self::HugeP3 => 2 
        }
    }

    /// Drop unused bits
    pub fn from_bits_drop(val: u8) -> Self {
        unsafe {*(&(val & 3) as *const _ as *const Self)}
    }
}

pub struct Frame {
    bits: usize 
}

impl Frame {
    
    pub(self) fn new(addr: PhysAddr, size: SizeType) -> Self{
        assert!(addr.is_page_aligned());
        Self {bits: addr.as_usize() | size as usize }
    }


    /// This function should be used carefully 
    pub(self) fn clone(&self) -> Self {
        Self { ..*self }
    }
    
    pub fn size(&self) -> SizeType {
        SizeType::from_bits_drop(self.bits as _)
    } 

    pub fn addr(&self) -> PhysAddr{
        // We make sure to drop the other bits
        PhysAddr::new(self.bits).align_lower(4096)
    }


}



impl fmt::Debug for Frame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Frame")
            .field("addr", &self.addr())
            .field("size", &self.size())
            .finish()
    }
}


#[cfg(test)]
pub mod test {
    use super::*;
    #[test_case]
    pub fn page(){
        let addr = PhysAddr::new(0xdeadc0de000);
        let size = SizeType::Page;

        let a = Frame::new(addr, size);
        assert_eq!(a.addr(), addr);
        assert_eq!(a.size(), size);

        let addr = PhysAddr::new(0xdeadbabe000);
        let size = SizeType::HugeP2;

        let a = Frame::new(addr, size);
        assert_eq!(a.addr(), addr);
        assert_eq!(a.size(), size);
        
    }
}
