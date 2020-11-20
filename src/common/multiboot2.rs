//! Parsing the Multiboot2 boot information

//! FIXME: the MultibootInfo struct should probably 
//! use the Pin mechanism, the ELFSymbols has `at()` fns
//! whereas the MemoryMap is indexable.. 
//! FIXME: overall better design could be achieved here.
//! 
//! This is purely from: 
//! https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Boot-information-format
//! I have not seen any other Multiboot2 parser, meaning that this may be 
//! some real, real bad code 
//! 
//! IMPORTANT NOTE: 
//! I discovered that GRUB2 DOES NOT respect the Multiboot2 spec
//! the ELFSymbols tag use 32 bit vals instead of 16bit.
//! for more info see the ELFSymbols struct.


crate::import_commons!();
use crate::*;

use core::{ops::Index, mem::size_of};

use common::{memory::PhysAddr, elf::SectionHeader64};
use common::elf::SectionHeader32;
use utils::maths::align_upper;


#[derive(Debug)]
pub struct TagHeader {
    pub type_id: u32,
    pub size: u32,
}

/// Represents a multiboot information 
pub struct MultibootInfo{
    pub size: u32,
    reserved: u32,
}

impl MultibootInfo {
    pub fn new(mbi_ptr: *const MultibootInfo) -> &'static Self{ 
        let mbi = unsafe {mbi_ptr.as_ref().unwrap()};
        assert_eq!(mbi.reserved, 0, 
            "Reserved field of the mbi should be zero, memory may have been overwritten");
        mbi
     }

    pub fn search_tag_from_type(&self, id: u32) -> Option<&'static TagHeader>{
        for tag in self {
            if tag.type_id == id {
                return Some(tag)
            }
        }
        None
    }

    pub fn content(&self) -> *const TagHeader{
        let ptr = self as *const _ as usize;
        (ptr + core::mem::size_of::<Self>()) as *const TagHeader
    }

    pub fn find<T>(&self) -> Option<&'static T>  where T: MBITag{
        T::find(self)
    }
}


impl IntoIterator for &MultibootInfo {
    type Item = &'static TagHeader;

    type IntoIter = MBIIterator;

    fn into_iter(self) -> Self::IntoIter {
        return MBIIterator {
            next_tag:self.content(),
            end: (self.content() as usize + self.size as usize) as _
        }
    }
}
pub struct MBIIterator {
    next_tag: *const TagHeader,
    end: *const TagHeader,
}

impl Iterator for MBIIterator {
    type Item = &'static TagHeader;

    /// Iters over the MBI tags
    /// doesn't include the end tag 
    fn next(&mut self) -> Option<Self::Item> {
 
        if self.end <= self.next_tag { return None }
        
        let tag = unsafe {&*self.next_tag};
        // This is the "ending" tag
        if tag.type_id == 0 && tag.size == 8 {
            return None;
        }
    

        self.next_tag = {
            let ptr = self.next_tag as usize;
            let offset = align_upper(tag.size as _, 8);
            (ptr+offset) as *const TagHeader
        };

        Some(tag)
    }
}




pub trait MBITag {
    /// This field is used for the auto impl 
    /// of the find trait
    fn find(mbi: &MultibootInfo) -> Option<&'static Self>;
}

/// This macro  modifies a struct :
///     - by adding derive(Debug) and repr(C) attributes
///     - by adding the id (type in the multiboot2 spec) and size fields 
///     - by implementing the MBITag trait
// See https://amanjeev.com/blog/rust-document-macro-invocations/
// for the $(#[$meta:meta])* trick 
macro_rules! info_tag {
    ( type = $type_id: literal,
        $(#[$meta:meta])*
        $name:ident { 
            $($element: ident: $ty: ty),*
        }
    ) =>  {
        $(#[$meta])*
        #[derive(Debug)] 
        #[repr(C)]
        pub struct $name { 
            type_id: u32,
            pub size: u32,
            $(pub $element: $ty),*
        }

        impl MBITag for $name {
            /// Search for the tag and cast the pointer to a ref to Self
            fn find(mbi:&MultibootInfo) -> Option<&'static Self> {
                let ptr = mbi.search_tag_from_type($type_id)?;
                unsafe {Some((ptr as *const  _ as *const Self).as_ref()?)}
            }
        }
    }
}

// Here we define all the tag structs


info_tag!( type = 4, 
    BasicMemoryInformation{
        mem_lower:  u32,
        mem_upper: u32
});
info_tag!( 
    type = 5,
    BIOSBootDevice{
        biosdev: u32,
        partition: u32,
        sub_partition: u32
});
info_tag!( 
    type = 1,
    BootCommandLine{
//    string: [u8]
});

impl BootCommandLine {
    pub fn string(&self) -> &'static str {
        let ptr = self as *const _ as  u64;
        let content =( ptr + 8) as *const u8;
        unsafe {
            let parts = core::slice::from_raw_parts(content, (self.size - 8) as _);
            core::str::from_utf8_unchecked(parts)
        }
    }
}

info_tag!( 
    type = 3,
    Modules{
        mod_start: u32,
        mod_end: u32
    //   string: [u8]
});
info_tag!( 
    type = 9,
    /// Be careful, this is not to spec as stated in 
    /// the (ref specification)[https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html]
    /// the entsize and shndx are 32bits in size but the 
    ELFSymbols{
        num: u16,
        entsize: u32,
        shndx: u32,
        reserved: u16
        

});

impl ELFSymbols {
    pub fn at(&self, index: usize) -> &'static SectionHeader64{
        if index > self.num as _ {panic!("Out of bound.");}
        let content = self as *const _  as usize + 20;
        let offset = index*size_of::<SectionHeader64>() as usize;
        unsafe {&*((content+offset) as  *const SectionHeader64)}
    }
    pub fn length(&self) -> usize {
        self.num as usize
    }
}
 

impl Index<usize> for &ELFSymbols {
    type Output = SectionHeader64;
    fn index(&self, index: usize) -> &Self::Output {
        self.at(index as _)
    }
}


info_tag!( 
    type = 6,
    MemoryMap {
        entry_size: u32,
        entry_version: u32
});

impl MemoryMap {
    pub fn nb_entries(&self) -> u32{
        (self.size - 16)/self.entry_size
    }
    pub fn at(&self, index:u32) -> &'static MemMapEntry {
        if !(index<self.nb_entries()){ panic!("Out of bounds."); }
        let content = self as *const _ as usize + size_of::<Self>();
        let offset = (index*self.entry_size) as usize;
        unsafe {&*((content + offset) as *const MemMapEntry)}
    }
}

impl Index<u32> for MemoryMap{
    type Output = MemMapEntry;
    fn index(&self, index: u32) -> &Self::Output {
        self.at(index)
    }
}

#[repr(C)]
pub struct MemMapEntry {
    pub base_addr: PhysAddr,
    pub length: u64,
    _type_id: u32,
    reserved: u32
}
impl MemMapEntry{
    pub fn type_id(&self) -> MemMapEntryType{
        if self._type_id == 2 || self._type_id > 5 {MemMapEntryType::Reserved}
        else {unsafe {core::mem::transmute(self._type_id)} }
    }
}
#[repr(u32)]
#[derive(Debug)]
/// Any non matching is reserved
pub enum MemMapEntryType {
    Reserved,
    RAM = 1,
    ACPI = 3,
    Preserve = 4,
    Defective = 5
}

info_tag!( 
    type = 2,
    BootLoaderName{
        //string 
    }
);
impl BootLoaderName {
    pub fn string(&self) -> &'static str {
        let ptr = self as *const _ as  u64;
        let content =( ptr + 8) as *const u8;
        unsafe {
            let parts = core::slice::from_raw_parts(content, (self.size - 8) as _);
            core::str::from_utf8_unchecked(parts)
        }
    }
}


info_tag!( 
    type =  10,
    APMTable {
        version:u16,             
        cseg:u16,                
        offset:u32,              
        cseg_16:u16,             
        dseg:u16,                
        flags:u16,               
        cseg_len:u16,            
        cseg_16_len:u16,         
        dseg_len:u16
});

info_tag!( 
    type = 7,
    VBEInfo {
        vbe_mode: u16,
        vbe_interface_seg: u16,
        vbe_interface_off: u16,
        vbe_interface_len: u16,
        vbe_control_info: [u8;512],
        vbe_mode_info: [u8;256]
});

info_tag!( 
    type = 8,
    FrameBufferInfo {
        framebuffer_addr: u64,
        framebuffer_pitch: u32,
        framebuffer_width: u32,
        framebuffer_height: u32,
        framebuffer_bpp: u8,
        framebuffer_type: u8,
        reserved: u8
});

info_tag!( 
    type = 11,
    EFI32SystemtablePtr {
        pointer: u32
});
info_tag!( 
    type = 12,
    EFI64SystemtablePtr {
        pointer: u64
});


info_tag!(
    type = 21,
    ImageLoadBasePhysicalAddress{
        load_base_addr: u32 
});





impl core::fmt::Debug for MemMapEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            MemMapEntry { base_addr, length, _type_id:_, reserved:_ } => {
                f.debug_struct("MemMapEntry")
                    .field("base_addr", &format_args!("{:?}", base_addr))
                    .field("length", &format_args!("{:#x}", length))
                    .field("type_id", &format_args!("{:?}", self.type_id()))
                    .finish()
} } }}
