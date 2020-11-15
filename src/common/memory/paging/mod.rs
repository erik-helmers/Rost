pub const PAGE_SIZE: usize = 4096;



pub mod mapper;


use super::{Frame, PhysAddr, SizeType, VirtAddr, alloc::FrameAllocator};

use core::ops::{Deref, DerefMut};

use crate::{utils::bitrange::BitRange, arch::paging::{*, PageDescriptorFlags as PDF }};

use mapper::Mapper;

/// Represents a page, fundamentally characterized by 
/// its size (4KiB, 2MiB, 1GiB) and its address. 
/// 
/// It is represented by a usize because
/// As the address must be at least, well, page aligned 
/// the lower bits of the address  be used freely (e.g. to store
/// the size)
#[derive(Debug, Copy, Clone)]
pub struct Page {
    bits: usize,
}

impl Page {

    /// The address must be aligned with the corresponding size
    pub fn new(addr: VirtAddr, size: SizeType) -> Self {

        assert!(addr.align_lower(size.size()) == addr,
                    "Addresss is not aligned correctly");

        Self {bits: addr.as_usize() | size as usize }
    }

    pub fn addr(&self) ->VirtAddr {
        // a correct page addr may never have set bits in it's lower part,
        // so we can safely use them 
        VirtAddr::new(self.bits).align_lower(SizeType::Page.size())
    }
    pub fn size(&self) -> SizeType {
        SizeType::from_bits_drop(self.bits as _ )
    }

}



pub struct ActivePageTable {
    mapper: Mapper<'static>
}

impl Deref for ActivePageTable {
    type Target = Mapper<'static>;
    fn deref(&self) -> &Mapper<'static> {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper<'static> {
        &mut self.mapper
    }
}

impl ActivePageTable {
    /// The reference to the table must be
    /// in recursive form 
    pub unsafe fn new(p4: &'static Table<Level4>) -> Self {
        Self {
            mapper: Mapper::new(p4)
        }
    }


    /// Creates a new recursivelly mapped 
    /// page at index entry `recursive_index`
    /// 
    /// Be careful the init function should not 
    /// use 
    /// 
    pub fn create_and_set_new_p4<A, F>(&mut self, recursive_index:u8,alloc: &mut A, init: F) 
    where A: FrameAllocator, F: FnOnce(&mut Mapper){
        let recursive_index = recursive_index as usize;
        // this assert is not strictly necessary but good to have for now
        assert!(self.p4().entries[recursive_index].is_unused());

        let new_p4 = alloc.allocate(SizeType::Page);
        
        // First step set recursive entry and zero
        {
            // FIXME: use future kernel heap, the address could break
            let p4_addr = VirtAddr::new(0o200_000_000_000_0000);
            let p4_page = Page::new(p4_addr, SizeType::Page);
            self.map_to(p4_page, new_p4.clone(), PDF::PRESENT | PDF::WRITABLE, alloc);
            let p4:&mut Table<Level1> = unsafe {&mut *(p4_addr.as_ptr_mut())};
            p4.zero();
            p4.entries[recursive_index].set(new_p4.addr(), PDF::PRESENT|PDF::WRITABLE);
            self.unmap(p4_page, alloc);
        }

        // The PML4 has now a recursive entry, let's initialize it 
        self.p4_mut().entries[recursive_index].set(new_p4.addr(), PDF::PRESENT|PDF::WRITABLE);

        let p4_addr = recursive_addr(recursive_index);
        let p4:&mut Table<Level4> = unsafe {&mut *(p4_addr.as_ptr_mut())};

        let mut mapper = unsafe {Mapper::new(p4)};
        init(&mut mapper);

        // safe: the p4 is valid
        unsafe {crate::arch::instructions::set_cr3(new_p4.addr())};


    }
}

pub struct InactivePageTable {
    p4_frame:Frame
}
impl InactivePageTable {
    pub fn new(frame: Frame) -> InactivePageTable {
        InactivePageTable {p4_frame: frame}
    }
}

/// The index is a number in [0;511], e.g. 243
/// 
/// The address returned is VirtAddr(243_243_243_243_0000)
pub fn recursive_addr(idx:usize) -> VirtAddr{
    let mut addr = 0;
    for _ in 0..4 {addr = (addr << 9) | idx }
    VirtAddr::new(addr << 12)
}


#[cfg(test)]
mod tests {

    crate::import_commons!();
    use super::*;

    #[test_case]
    pub fn page(){
        let addr = VirtAddr::new(0o667_123_123_123_0000);
        let size = SizeType::Page;

        let a = Page::new(addr, size);
        assert_eq!(a.addr(), addr);
        assert_eq!(a.size(), size);
        let addr = VirtAddr::new(0o042_000_000_0000);
        let size = SizeType::HugeP2;

        let a = Page::new(addr, size);
        assert_eq!(a.addr(), addr);
        assert_eq!(a.size(), size);


    }
}
