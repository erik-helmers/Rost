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


pub fn translate(p4: &Table<Level4>, addr: VirtAddr) ->Option<PhysAddr>{
    let offset = addr.as_usize() % PAGE_SIZE;
    translate_page(
        p4, Page::containing_address(addr)
    ).map(|addr| addr + offset)
}


fn translate_page(p4: &Table<Level4>, page: Page) -> Option<PhysAddr> {

    let p3 = RPT::new(p4).next_table(page.table_index(3));

    p3.and_then(|p3| RPT::new(p3).next_table(page.table_index(1)))
      .and_then(|p2| RPT::new(p2).next_table(page.table_index(1)))
      .and_then(|p1| p1.entries[page.table_index(0)].base_addr())

}




pub type RP4 = RecursivePageTable<'static, Level4>;
type RPT<'a, T> = RecursivePageTable<'a, T>;

#[repr(transparent)]
pub struct RecursivePageTable<'a, T: TablePointerLevel> {
    table: &'a Table<T>
}

impl<'a, T: TablePointerLevel> RecursivePageTable<'a, T> {

    pub fn new(table: &'a Table<T>) -> Self {
        Self {table}
    }

    fn next_table(&self, index: usize) -> Option<&'a Table<T::Next>> {
        let addr = self.next_table_address(index)?;
        // We have the guaranty that the table is recursively mapped
        Some(unsafe {&*(addr.as_ptr())})
    }    

    fn next_table_address(&self, index: usize) -> Option<VirtAddr>{
        if !self.table.entries[index].present() {return None;}
        if self.table.entries[index].huge() {return None;}
        let table_ptr = self.table as *const _ as usize;
        Some(VirtAddr::new(((table_ptr<<9) | 0o177_777_000_000_000_000_0000) | (index << 12)))
    }
}

impl<'a, T: TablePointerLevel> Into<&'a Table<T>> for RecursivePageTable<'a, T> {
    fn into(self) -> &'a Table<T> {
        self.table
    }
}


