use crate::*;
use super::*;

use crate::utils::bitrange::BitRange;

use common::memory::*;
use alloc::FrameAllocator;

use arch::paging::{Table, Level4, PageDescriptorFlags as PDF};



pub struct Mapper<'a> {
    p4: &'a Table<Level4>
}

impl<'a> Mapper<'a> {

    /// Safety: 
    /// The passed reference should be of a recursively
    /// mapped page table
    pub unsafe fn new(table: &'a Table<Level4>) -> Self {
        Self { p4:table }
    }

    pub fn p4(&self) -> &Table<Level4>{
        self.p4 
    }

    pub fn p4_mut(&mut self) -> &mut Table<Level4>{
        unsafe{&mut *(self.p4 as *const _ as usize as *mut Table<Level4>)}
    }

    /// translates a virtual address to phys
    /// if it is mapped. this function supports huge pages.
    pub fn translate(&self, addr: VirtAddr) ->Option<PhysAddr>{

        let mut table = self.p4().downcast();

        // Loop through each level
        // exiting with value if the page is huge,
        // or none if the page isn't present
        for level in (0..=3).rev() {

            let idx = addr.table_index(level);
            let descr_next = &table.entries[idx];
            if !descr_next.flags().contains(PDF::PRESENT) {
                // there is no mapped physaddr
                return None;
            }
            if level == 0 || descr_next.flags().contains(PDF::HUGE) {
                let base = descr_next.base_addr()?.as_usize();
                let offset = addr.as_usize().get_bits(0..12+9*level);
                return Some(PhysAddr::new(base+offset));
            }
            
            table = table.next_table(idx)?;
        }

        unreachable!()
    }


    pub fn map<A>(&mut self, page: Page, flags: PDF, allocator: &mut A)
        where A: FrameAllocator
    {
        let frame = allocator.allocate(page.size());
        self.map_to(page, frame, flags, allocator)
    }


    pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: PDF,
        allocator: &mut A)
    where A: FrameAllocator
    {
        assert_eq!(frame.size(), SizeType::Page);
        assert_eq!(frame.size(), page.size());

        let addr = page.addr();
        let map_lvl = page.size().level() as usize;

        // a p4 entry can not be huge
        let mut table = self.p4_mut().downcast_mut();

        // Loop through each level, where we need a table,
        // i.e. we try to allocate 2MiB, we can just
        // stop after potentially creating the p3 and p2
        for level in (map_lvl+1..=3).rev() {
            let idx = addr.table_index(level);
            table = table.next_table_create(idx, allocator);
        }

        let flags = if map_lvl == 0 {flags} else {flags | PDF::HUGE};
        table.entries[addr.table_index(map_lvl)].set(frame.addr(), flags)
        
    }

    pub fn unmap<A>(&mut self, page: Page, allocator: &mut A)
        where A: FrameAllocator
    {
        assert!(self.translate(page.addr()).is_some());

        let addr = page.addr();
        let map_lvl = page.size().level() as usize;
        let mut table = self.p4_mut().downcast_mut();

        for level in (map_lvl+1..=3).rev() {
            let idx = addr.table_index(level);
            // FIXME: the page may be mapped through
            // a huge page, causing an unwrap error
            table = table.next_table_mut(idx).expect(
                "the page was invalid & mapped through a huge page"
            );
        }

        
        let phys_addr = table.entries[addr.table_index(0)].base_addr().unwrap();
        table.entries[addr.table_index(0)].unused();

        // TODO: free p(1,2,3) table if empty?
        allocator.deallocate(Frame::new(phys_addr, page.size()));
    }

}


impl<T: TablePointerLevel> Table<T> {

    /// Returns a reference to the table described
    /// at the `index`-nth  entry if it is exists.
    pub fn next_table(&self, index: usize)
            -> Option<&Table<T::Next>> {
        let addr = self.next_table_address(index)?;
        Some(unsafe {&*(addr.as_ptr())})
    }

    pub fn next_table_create<A>(&mut self,
        index: usize,
        allocator: &mut A)
        -> &mut Table<T::Next>
    where A: FrameAllocator
    {
            if self.next_table(index).is_none() {
                let frame = allocator.allocate(SizeType::Page);
                self.entries[index].set(frame.addr(), PDF::PRESENT | PDF::WRITABLE);
                self.next_table_mut(index).unwrap().zero();
            }
            self.next_table_mut(index).unwrap()
    }

    /// Returns a reference to the table described
    /// at the `index`-nth  entry if it is exists.
    pub fn next_table_mut(&mut self, index: usize)
            -> Option<&mut Table<T::Next>> {
        let addr = self.next_table_address(index)?;
        Some(unsafe {&mut *(addr.as_ptr_mut())})
    }

    /// Returns an address poiting to the table described
    /// at the `index`-nth  entry if it is exists.
    pub fn next_table_address(&self, index: usize)
            -> Option<VirtAddr> where T: TablePointerLevel{
        if !self.entries[index].flags().contains(PDF::PRESENT ) {return None;}
        if self.entries[index].flags().contains(PDF::HUGE ) {return None;}

        let table_ptr = self as *const _ as usize;
        Some(VirtAddr::new_dropping(table_ptr << 9 | index << 12))
    }
}
