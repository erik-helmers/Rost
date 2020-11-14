//! This module is the arch specific 
//! definition of page tables and 
//! their entry description. 
//! 
//! This module is used only by the common::memory module



use crate::*;
use common::memory::{VirtAddr, PhysAddr, Frame};
use utils::bitflags;



use core::{marker::PhantomData, fmt};

pub trait TableLevel {}

/// Page Map Level 4
/// 
/// Huge bit can not be set
pub struct Level4;
/// Page Directory Pointer
pub struct Level3;
/// Page Directory
pub struct Level2;
/// Page Table
/// 
/// Huge bit can not be set
pub struct Level1;
/// used when type is unknown
pub struct Unknown;

impl TableLevel for Level4{}
impl TableLevel for Level3{}
impl TableLevel for Level2{}
impl TableLevel for Level1{}
impl TableLevel for Unknown{}

/// Represents a TableLevel that can point to another table
/// (i.e. that is not a Page Table)
pub trait TablePointerLevel : TableLevel{
    type Next: TableLevel;
}

impl TablePointerLevel for Level4 {
    type Next = Level3 ;
}
impl TablePointerLevel for Level3 {
    type Next = Level2;
}
impl TablePointerLevel for Level2 {
        type Next = Level1;
}

impl TablePointerLevel for Unknown {
    type Next = Unknown;
}




#[repr(C, align(0x1000))]
/// This represents either a 
///   - Page map level 4 (PML4)
///   - Page directory pointer (PDP)
///   - Page directory (PD)
///   - Page table (PT)
pub struct Table<L: TableLevel> {
    pub entries: [PageDescriptor; 512],
    __level: PhantomData<L>
}

impl<L> Table<L> where L: TableLevel {
    pub fn zero(&mut self){
        for entry in &mut self.entries {
            entry.unused();
        }
    }

    /// Downcasts Table to a table of unkown level
    pub fn downcast(&self) -> &Table<Unknown> {
        unsafe {&*(self as *const _ as usize as *const Table<Unknown>)}
    }

}


/// When trying to access a table's child
/// the following may occur
pub enum PageAccessError{
    IsHuge,
    IsClear,
}



impl PageDescriptor {
    
    pub fn zero() -> Self{
        Self{bits:0}
    }

    /// Clears the entry
    pub fn unused(&mut self){
        self.bits = 0;
        
    }
    /// A clear page can safely be used 
    pub fn is_unused(&self) -> bool {
        self.bits == 0
    }

    pub fn flags(&self) -> PageDescriptorFlags {
        PageDescriptorFlags::from_bits_truncate(self.bits)
    }

    /// Returns the address physical pointed by
    /// this descriptor
    pub fn base_addr(&self) -> Option<PhysAddr> {
        if self.flags().contains(PageDescriptorFlags::PRESENT) {
            Some(PhysAddr::new( 0x000fffff_fffff000 & self.bits as usize ))
        } else {None} 
    }

    pub fn set(&mut self, addr: PhysAddr, flags: PageDescriptorFlags){
        assert!(addr.as_usize() & !0x000fffff_fffff000 == 0);
        self.bits = (addr.as_usize() as u64) | flags.bits;

    }

    /// Sets the base address pointed by this 
    /// descriptor and sets the Present flag
    pub fn set_base_addr(&mut self, addr: PhysAddr) {
        assert_eq!(addr.align_lower(4096), addr);
        self.bits |= PageDescriptorFlags::PRESENT.bits;
        self.bits |= addr.as_usize() as u64;
    }

    
}

impl fmt::Debug for PageDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PageDescriptor")
            .field("addr", &self.base_addr())
            .field("flags", &format_args!("{:#b}", self.bits))
            .finish()
    }
}



pub struct PageDescriptor{
    bits: u64   
}

impl PageDescriptor {

/*
    /// These bits are not interpreted by the processor
    /// and are available for use by system software.
    pub os_spec_lo(): Val(9..=11);
    

    _base_addr: Val(12..=51);
    /// These bits are not interpreted by the processor and are 
    /// available for use by system software.
    pub os_spec_hi: Val(52..=62);
 */
}




crate::bitflags!(
    pub struct PageDescriptorFlags(u64){
    /// Present (P) Bit.  Bit 0. This bit indicates whether the page-translation
    /// table or physical page is loaded in physical memory. When the P bit is
    /// cleared to 0, the table or physical page is not loaded in physical
    /// memory. When the P bit is set to 1, the table or physical page is loaded
    ///  in physical memory.
    const PRESENT = 1 << 0;
    ///This bit controls read/write access to all physical pages mapped by the
    /// table entry. For example, a page-map level-4 R/W bit controls read/write 
    /// access to all 128M (512 × 512 × 512) physical pages it maps through the 
    /// lower-level translation tables. When the R/W bit is cleared to 0, access
    /// is restricted to read-only. When the R/W bit is set to 1, both read and
    /// write access is allowed.
    const WRITABLE = 1 << 1;
    /// This bit controls user (CPL 3) access to all physical pages
    ///  mapped by the table entry. For example, a page-map level-4 U/S bit 
    /// controls the access allowed to all 128M (512 × 512 × 512) physical pages
    ///  it maps through the lower-level translation tables. When the U/S bit is
    ///  cleared to 0, access is restricted to supervisor level (CPL 0, 1, 2).
    ///  When the U/S bit is set to 1, both user and supervisor
    ///  access is allowed. 
    const USER_ACCESSIBLE = 1 << 2;
    /// Page-Level Writethrough (PWT) Bit. This bit indicates whether 
    /// the page-translation table or physical page to which this entry points
    ///  has a writeback or writethrough caching policy. When the PWT bit is
    ///  cleared to 0, the table or physical page has a writeback caching
    ///  policy. When the PWT bit is set to 1, the table or physical page has
    ///  a writethrough caching policy. 
    const PL_WRITETHROUGHT = 1 << 3;
    
    ///Page-Level Cache Disable (PCD) Bit. 
    /// 
    ///  This bit indicates whether the
    ///  page-translation table or physical page to which this entry points is 
    ///  cacheable. When the PCD bit is cleared to 0, the table or physical page
    ///  is cacheable. When the PCD bit is set to 1, the table or physical page
    ///  is not cacheable.
    const PL_CACHEDISABLE = 1 << 4;

    /// Accessed (A) Bit.  
    /// 
    /// This bit indicates whether the page-translation
    /// table or physical page to which this entry points has been accessed. The
    /// A bit is set to 1 by the processor the first time the table or physical
    /// page is either read from or written to. The A bit is never cleared by 
    /// the processor. Instead, software must clear this bit to 0 when it needs 
    /// to track the frequency of table or physical-page accesses.
    const ACCESSED = 1 << 5;
    ///Dirty (D) Bit. 
    /// 
    ///  This bit is only present in the lowest level of the page-translation 
    /// hierarchy. It indicates whether the physical page to which this entry
    ///  points has been written. The D bit is set to 1 by the processor the 
    /// first time there is a write to the physical page. The D bit is never
    ///  cleared by the processor. Instead, software must clear this bit to 0 
    /// when it needs to track the frequency of physical-page writes
    const DIRTY = 1 << 6;

    ///Page Size (PS) Bit.
    ///
    ///  This bit is present in page-directory entries
    ///  and long-mode page-directory-pointer entries. When the PS bit is set in
    ///  the page-directory-pointer entry (PDPE) or page-directory entry (PDE), 
    ///  that entry is the lowest level of the page-translation hierarchy. When the
    ///  PS bit is cleared to 0 in all levels above PTE, the lowest level of the
    ///  page-translation hierarchy is the page-table entry (PTE), and the
    ///  physical-page size is 4 Kbytes. The physical-page size is determined
    ///  as follows:
    ///      - If EFER.LMA=1 and PDPE.PS=1, the physical-page size is 1
    ///          Gbyte.
    ///      - If CR4.PAE=0 and PDE.PS=1, the physical-page size is 4 Mbytes.
    ///      - If CR4.PAE=1 and PDE.PS=1, the physical-page size is 2 Mbytes.

    const HUGE = 1 << 7;
    ///Global Page (G) Bit.
    /// 
    /// This bit is only present in the lowest level of the page-translation
    /// hierarchy. It indicates the physical page is a global page. The TLB
    /// entry for a global page (G=1) is not invalidated when CR3 is loaded 
    /// either explicitly by a MOV CRn instruction or implicitly during a task
    /// switch. Use of the G bit requires the page-global enable bit in CR4 to
    /// be set to 1 (CR4.PGE=1).
    const GLOBALPAGE = 1 << 8;

    ///No Execute (NX) Bit. 
    /// 
    /// This bit is present in the translation-table entries defined for PAE
    /// paging, with the exception that the legacy-mode PDPE does not contain 
    /// this bit. This bit is not supported by non-PAE paging.
    const NO_EXECUTE = 1 << 63;

    }

);
