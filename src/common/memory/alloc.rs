//! This module provides a trait for a
//! FrameAllocator and a simple implementation

use core::cmp::max;

use crate::*;

use super::{Frame, PhysAddr, SizeType};
use common::multiboot2::{ELFSymbols, MemoryMap, MemMapEntryType};

pub trait FrameAllocator {
        /// Allocate a frame
        ///
        /// Panics if requested size can not be allocated
        fn allocate(&mut self, size: SizeType) -> Frame {
            self.try_allocate(size).expect(
                "Out of memory."
            )
        }

        /// Try to allocate a frame
        ///
        /// If the current size cannot be allocated
        /// return None
        fn try_allocate(&mut self, size: SizeType) -> Option<Frame>;

        /// Deallocate a given frame
        ///
        /// Panics if the Frame was not allocated
        fn deallocate(&mut self, frame: Frame);
}

pub struct IncrAllocator<'a> {
    mmap: &'a MemoryMap,
    _elfs: &'a ELFSymbols,
    pub next_addr: PhysAddr,
}


impl<'a> IncrAllocator<'a> {
   pub fn new(mmap: &'a MemoryMap, elfs: &'a ELFSymbols) -> Self{
       Self {
           mmap: mmap,
           _elfs: elfs,
           next_addr: PhysAddr::null()
       }
   }

    /// Returns the next address valid for the mmap sections
   /// If no section is found returns None
   pub fn next_valid_mmap(&self, size: usize) -> Option<PhysAddr>{
       let next = self.next_addr;
       let mmap = self.mmap;
       for i in 0..mmap.nb_entries() {

           let sec_start = mmap[i].base_addr;
           let sec_end = sec_start + mmap[i].length as usize;
           let sec_size = (sec_end - sec_start.as_usize()).as_usize();

           match mmap[i].type_id() {

               MemMapEntryType::RAM =>
                   // if the frame would overflow on a reserved section
                   // (it is assumed there are no contiguous valid ram section)
                   if sec_end < next+size { continue }
                   else if next <= sec_start && size <= sec_size  {
                        return Some(sec_start) 
                    }
                   else { return Some(next) },
               _ => continue,
           }
       };
       None
   } 

   pub fn next_valid_elf(&self, _size: usize) -> Option<PhysAddr>{
       // FIXME: dummy impl
       // for now we just hope for the best
       if self.next_addr.as_usize() <= 0x20_000 {
           return Some(PhysAddr::new(0x20_000))
        };
       return Some(self.next_addr);
   }
}


impl<'a> FrameAllocator for IncrAllocator<'a> {


    fn deallocate(&mut self, _frame: Frame) {
        //FIXME:
    }

    fn try_allocate(&mut self, size: SizeType) -> Option<Frame> {
        // FIXME: the frame may not be properly aligned
        let n = loop {
            let a = self.next_valid_elf(size.size())?;
            let b = self.next_valid_mmap(size.size())?;
            if a==b {break a};
            self.next_addr = max(a,b);
        };

        self.next_addr = n + size.size();
        Some(Frame::new(n, size))
    }
}
