pub const PAGE_SIZE: usize = 4096;

pub use super::{VirtAddr, PhysAddr, Frame, alloc::FrameAllocator};
use crate::arch::paging::{*, PageDescriptorFlags as PDF };


/// Represents a page
#[derive(Debug, Copy, Clone)]
pub struct Page {
    addr: VirtAddr,

}

impl Page {
    pub fn new(addr: VirtAddr) -> Self {
        Self {addr: addr.align_lower(PAGE_SIZE) }
    }
    pub fn as_virt(&self) ->VirtAddr {
        self.addr
    }
}


pub use active_pt::*;

#[cfg(feature="recursive_mapping")]
mod active_pt {
    use core::marker::PhantomData;

    use super::*;
    use crate::utils::bitrange::BitRange;

    pub struct ActivePageTable<'a> {
        p4: &'a Table<Level4>
    }

    impl<'a> ActivePageTable<'a> {
        pub unsafe fn new(p4: &'a Table<Level4>) -> Self {
            Self{p4}
        }
    }

    impl<'a> ActivePageTable<'a>{

        pub fn p4_mut(&mut self) -> &mut Table<Level4>{
            unsafe{&mut *(self.p4 as *const _ as usize as *mut Table<Level4>)}
        }
        /// translates a virtual address to phys
        /// if it is mapped. this function supports huge pages.
        pub fn translate(&self, addr: VirtAddr) ->Option<PhysAddr>{

            let mut table = self.p4.downcast();

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

        /// this function does not support to huge pages
        pub fn map<A>(&mut self, page: Page, flags: PDF, allocator: &mut A)
            where A: FrameAllocator
        {
            let frame = allocator.allocate(PAGE_SIZE).expect("out of memory");
            self.map_to(page, frame, flags, allocator)
        }


        /// this function does not support to huge pages
        pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: PDF,
            allocator: &mut A)
        where A: FrameAllocator
        {
            let p4 = self.p4_mut();
            let addr = page.addr;
            let p3 = p4.next_table_create(addr.table_index(3), allocator);
            let p2 = p3.next_table_create(addr.table_index(2), allocator);
            let p1 = p2.next_table_create(addr.table_index(1), allocator);

            assert!(p1.entries[addr.table_index(0)].is_unused());
            p1.entries[addr.table_index(0)].set(frame.to_phys(), flags | PDF::PRESENT);
        }

        pub fn unmap<A>(&mut self, page: Page, allocator: &mut A)
            where A: FrameAllocator
        {
            assert!(self.translate(page.addr).is_some());
            let addr = page.addr;
            let p1 = self.p4_mut()
                        .next_table_mut(addr.table_index(3))
                        .and_then(|p3| p3.next_table_mut(addr.table_index(2)))
                        .and_then(|p2| p2.next_table_mut(addr.table_index(1)))
                        .expect("mapping code does not support huge pages");
            let frame = p1.entries[addr.table_index(0)].base_addr().unwrap();
            p1.entries[addr.table_index(0)].unused();
            // TODO free p(1,2,3) table if empty
            allocator.deallocate(Frame::new(frame));
        }

    }


    /// This struct may only be used by a recursively mapped
    /// active page table


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
                    assert!(!self.entries[index].flags().contains(PDF::HUGE),
                    "mapping code does not support huge pages");
                    let frame = allocator.allocate(4096).expect("no frames available");
                    self.entries[index].set(frame.to_phys(), PDF::PRESENT | PDF::WRITABLE);
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

}









