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
use core::ops::Index;

use crate::utils::maths::align_upper;
#[repr(C)]
struct Header (pub u32, pub u32);

#[derive(Debug)]
pub struct TagHeader {
    pub type_id: u32,
    pub size: u32,
}

/// Represents a multiboot information 
pub struct MultibootInfo( *const() );

impl MultibootInfo {
    pub fn new(mbi: *const()) -> Self{ Self(mbi) }

    /// Returns the `total_size` field per multiboot spec
    pub fn total_size(&self) -> u32{
        let ptr = self as *const _  as *const u32;
        return unsafe {*ptr};
    }
    pub fn search_tag_from_type(&self, id: u32) -> Option<*const ()>{
        for ptr in self {
            let tag = unsafe {ptr.as_ref().unwrap()};
            if tag.type_id == id {
                return Some(ptr as *const() )
            }
        }
        None
    }

    pub fn find<T>(&self) -> Option<&T>  where T: MBITag{
        T::find(self)
    }

    
}


impl IntoIterator for &MultibootInfo {
    type Item = *const TagHeader;

    type IntoIter = MBIIterator;

    fn into_iter(self) -> Self::IntoIter {
        let total_size: u32= {
            let hdr = unsafe {self.0.cast::<Header>().as_ref().unwrap()};
            assert_eq!(hdr.1, 0, "The bootinfo ptr may be invalid, or memory has been overwritten");
            hdr.0
        };
        return MBIIterator {
            mbi:self.0,
            // the header is 64bit 
            cur_pos: 8,
            size:total_size
        }
    }
}
pub struct MBIIterator {
    mbi: *const(),
    size: u32,
    cur_pos: u32
}

impl Iterator for MBIIterator {
    type Item = *const TagHeader;

    /// Iters over the MBI tags
    /// doesn't include the end tag 
    fn next(&mut self) -> Option<Self::Item> {
 
        if self.cur_pos >= self.size { return None }
        
        let ptr = self.mbi as u64;

        let tag = {
            let tag_ptr  = (ptr+self.cur_pos as u64) as *const TagHeader;
            unsafe {tag_ptr.as_ref().unwrap()}
        };
        // This is the "ending" tag
        if tag.type_id == 0 && tag.size == 8 {
            return None;
        }

        self.cur_pos += align_upper(tag.size as _, 8) as u32 ;

        Some(tag)
    }
}


pub trait MBITag {
    fn find(mbi: &MultibootInfo) -> Option<&Self>;
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
            fn find(mbi:&MultibootInfo) -> Option<&Self> {
                let ptr = mbi.search_tag_from_type($type_id)?;
                unsafe {Some((ptr as *const Self).as_ref()?)}
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
use super::elf::SectionHeader64;
use super::elf::SectionHeader32;

impl ELFSymbols {
    pub fn at(&self, i: usize) -> Option<&SectionHeader64>{
        if i > self.num as _ {return None}
        let content = self as *const _  as usize + 20;
        let section = (content +  i*0x40) as *const u8;
        Some(unsafe {section.cast::<SectionHeader64>().as_ref().unwrap()})
    }
    pub fn at32(&self, i: usize) -> Option<&SectionHeader32>{
        if i > self.num as _ {return None}
        let ptr = self as *const _  as usize;
        let content = (ptr + i*core::mem::size_of::<SectionHeader32>()) as *const u8;
        Some(unsafe {content.cast::<SectionHeader32>().as_ref().unwrap()})
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
}

impl Index<u32> for MemoryMap{
    type Output = MemMapEntry;

    fn index(&self, index: u32) -> &Self::Output {
        unsafe {
            if !(index<self.nb_entries()){ panic!("Out of bounds."); }
            let ptr = self as *const _ as usize;
            // We don't forget to add the offset of the 4 first u32
            let content = ptr + 16;
            let offset = (index*self.entry_size) as usize;
            let ptr_mem_entry: *const MemMapEntry = (content + offset) as _;
            ptr_mem_entry.as_ref().unwrap()
        }

    }
}
#[repr(C)]
pub struct MemMapEntry {
    base_addr: u64,
    length: u64,
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
                    .field("base_addr", &format_args!("{:#016x}", base_addr))
                    .field("length", &format_args!("{:#x}", length))
                    .field("type_id", &format_args!("{:?}", self.type_id()))
                    .finish()
} } }}
