use crate::*;

pub mod paging;
pub mod utils;

pub use utils::{VirtAddr, PhysAddr};



bitstruct!{
    /// Repersents a physical frame
    /// 
    /// It is mainly used as an ID for FrameAlloc and paging purposes
    /// Because of this use, it is "unsafe" to clone or copy them :
    /// the only way to create them should be via a FrameAllocator.
    /// However, for easier use, the memory module has access to a
    /// "clone()" function.
    /// 
    /// As each frame must at least be page sized (4096 bytes on x86)
    /// and page aligned we can use the "unused" bytes for 
    /// OS related purpose.
    pub struct Frame(usize) {
        _addr: Val(0..64);
    }
}


impl Frame {
    
    pub(self) fn new(addr: PhysAddr) -> Self{
        assert!(addr.is_page_aligned());
        Self {bits: addr.as_usize() }
    }


    /// This function should be used carefully 
    pub(self) fn clone(&self) -> Self {
        Self { ..*self }
    }
    
    pub fn to_phys(&self) -> PhysAddr{
        PhysAddr::new(self._addr() as usize)
    }


}
