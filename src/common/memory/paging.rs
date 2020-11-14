pub const PAGE_SIZE: usize = 4096;

pub use super::{VirtAddr, PhysAddr, Frame};


use crate::arch::paging::*;

/// Represents a page 
#[derive(Debug, Copy, Clone)]
struct Page {
    addr: VirtAddr
}

impl Page {
    pub fn containing_address(addr: VirtAddr) -> Self {
        Page {addr: addr.align_lower(PAGE_SIZE)}
    }
}
impl Page {
    pub fn table_index(&self, level: u8) -> usize {
        (self.addr.as_usize() >> (9*level+12)) & 0o777
    }        
}



pub use active_pt::*;

#[cfg(feature="recursive_mapping")]
mod active_pt {
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
        /// translates a virtual address to phys
        /// if it is mapped.
        pub fn translate(self, addr: VirtAddr) ->Option<PhysAddr>{

            let mut table = self.p4.downcast();
            
            // Loop through each level
            // exiting with value if the page is huge,
            // or none if the page isn't present 
            for level in (0..=3).rev() {
                
                let idx = addr.table_index(level);
                let descr_next = &table.entries[idx];
                if !descr_next.present() {
                    // there is no mapped physaddr
                    return None;
                }
                if descr_next.huge() {
                    let base = descr_next.base_addr()?.as_usize();
                    let offset = addr.as_usize().get_bits(0..12+9*level);
                    return Some(PhysAddr::new(base+offset));
                }

                table = unsafe {RPT::new(table).next_table(idx)?};
            }

            
            Some(table.entries[addr.table_index(0)].base_addr()?+addr.offset())
        }
    

    }
    
    type RPT<'a, T> = RecursivePageTable<'a, T>;

    #[repr(transparent)]
    /// This struct may only be used by a recursively mapped
    /// active page table 
    struct RecursivePageTable<'a, T: TablePointerLevel> {
        table: &'a Table<T>
    }

    impl<'a, T: TablePointerLevel> RecursivePageTable<'a, T> {
        
        /// Creates a table accessor using recursive mapping
        /// 
        /// Safety:
        /// 
        /// The reference should use the "recursive addressing", 
        /// i.e. if the P4 maps to itself at entry 0oXXX, and we try 
        /// to address a P2 table the addr should be something like:
        ///      `0oXXX_XXX_132_465_0000`
        /// 
        pub unsafe fn new(table: &'a Table<T>) -> Self {
            Self {table}
        }


        /// Returns a reference to the table described
        /// at the `index`-nth  entry if it is exists.
        pub fn next_table(&self, index: usize)
                -> Option<&'a Table<T::Next>> {
            let addr = self.next_table_address(index)?;
            Some(unsafe {&*(addr.as_ptr())})
        }

        /// Returns an address poiting to the table described
        /// at the `index`-nth  entry if it is exists.
        pub fn next_table_address(&self, index: usize) 
                -> Option<VirtAddr> where T: TablePointerLevel{
            if !self.table.entries[index].present() {return None;}
            if self.table.entries[index].huge() {return None;}
            let table_ptr = self.table as *const _ as usize;
            Some(VirtAddr::new_dropping(table_ptr << 9 | index << 12))
        }
    }

}









