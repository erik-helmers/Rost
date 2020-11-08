//! Parsing the Multiboot2 boot information

//! Yes, I know this is not really a device.
//! Some may even say, it's not a device AT ALL.
//! We'll, to this I would respond the following : 
//! 
//! TODO: where should this be ?
//! 
//! This is purely from: 
//! https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Boot-information-format
//! I have not seen any other Multiboot2 parser, meaning that this may be 
//! some real, real bad code 


crate::import_commons!();
use core::ops::Index;

use crate::utils::maths::align_upper;

/// This is a mini macro that modifies a struct :
///     - by adding derive(Debug) and repr(C) attributes
///     - by adding the id (type), tags, and size fields 
// See https://amanjeev.com/blog/rust-document-macro-invocations/
// for the $(#[$meta:meta])* trick 
macro_rules! info_tag {
    (
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
            size: u32,
            $($element: $ty),*
        }
    }
}
/// Exactly the same but without derive(Debug)
macro_rules! info_tag_no_dbg {
    (
        $(#[$meta:meta])*
        $name:ident { 
            $($element: ident: $ty: ty),*
        }
    ) => {
        #[repr(C)]
        pub struct $name { 
            type_id: u32,
            size: u32,
            $($element: $ty),*
        }
    }
}

#[repr(u32)]
pub enum Tags {
    BasicMemoryInformation(BasicMemoryInformation) = 4,
    BIOSBootDevice(BIOSBootDevice) = 5,
    MemoryMap(*const MemoryMap) = 6
}


info_tag!( 
    /// type = 4
    BasicMemoryInformation{
    mem_lower:  u32,
    mem_upper: u32
});
info_tag!( 
    /// type = 5
    BIOSBootDevice{
    biosdev: u32,
    partition: u32,
    sub_partition: u32
});
info_tag!( 
    /// type = 1
    BootCommandLine{
//    string: [u8]
});
info_tag!( 
    /// type = 3
    Modules{
    mod_start: u32,
    mod_end: u32
 //   string: [u8]
});
info_tag!( 
    /// type = 9
    ELFSymbols{
    num: u16,
    entsize: u16,
    shndx: u16,
    reserved: u16
});

info_tag!( 
    /// type = 6
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
    /// type = 2
    BootLoaderName {
//    string: [u8]
});
info_tag!( 
    /// type =  10
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
    /// type = 7
    VBEInfo {
    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
    vbe_control_info: [u8;512],
    vbe_mode_info: [u8;256]
});

info_tag!( 
    /// type = 8
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
    /// type = 11
    EFI32SystemtablePtr {
    pointer: u32
});
info_tag!( 
    /// type = 12
    EFI64SystemtablePtr {
    pointer: u64
});


info_tag!(ImageLoadBasePhysicalAddress{
    load_base_addr: u32 
});



#[repr(C)]
struct Header (pub u32, pub u32);


struct TagHeader {
    pub type_id: u32,
    pub size: u32,
}


unsafe fn search_tag() {

}

unsafe fn display_mmap(mmap: &MemoryMap){
    
    
    serial_println!("{:#?}", mmap);
    serial_println!("========= ENTRIES: ===========");
    for i in 0..mmap.nb_entries(){
        serial_println!("Memory map entry #{}: {:#?}",i,  mmap[i]);
    }
    
}
unsafe fn parse_boot_info_tag(tag: &TagHeader) -> Option<()>{
    serial_print!("Found header type {} size {} : ", tag.type_id, tag.size);
    
    
    let ptr = tag as *const TagHeader;
    if tag.type_id == 6 {
        display_mmap(ptr.cast::<MemoryMap>().as_ref()?);
        return Some(());
    } return Some(());
    
    let content : &dyn core::fmt::Debug = match tag.type_id {
         1 => ptr.cast::<BootCommandLine>() .as_ref()?,
         3 => ptr.cast::<Modules>() .as_ref()?,
         9 => ptr.cast::<ELFSymbols>() .as_ref()?,
         6 => &ptr.cast::<MemoryMap>() .as_ref()?[0],
         2 => ptr.cast::<BootLoaderName>() .as_ref()?,
        10 => ptr.cast::<APMTable>() .as_ref()?,
         7 => ptr.cast::<VBEInfo>() .as_ref()?,
         8 => ptr.cast::<FrameBufferInfo>() .as_ref()?,
        21 => ptr.cast::<ImageLoadBasePhysicalAddress>() .as_ref()?,


        _ => &"Unknown tag type",
    };
    serial_println!("{:#?}", content);

    Some(())

}

pub fn parse_boot_info(ptr: *const u8) -> Option<()>{
    let mut uptr = ptr as u64;
    // Better pray the pointer is valid
    assert_eq!(align_upper(ptr as usize, 8), ptr as usize);
    
    let total_size: u32= {
        let hdr = unsafe {ptr.cast::<Header>().as_ref()?};
        assert_eq!(hdr.1, 0, "The bootinfo ptr may be invalid, or memory has been overwritten");
        hdr.0
    };

    let raw = unsafe { core::slice::from_raw_parts(ptr, total_size as usize) };
    
    // We've already read 64bits
    let mut cur_pos = 8;
    loop {
        uptr = ptr as u64 + cur_pos as u64;
        let tag = unsafe{ ptr.add(cur_pos).cast::<TagHeader>().as_ref()?};
        let padded = align_upper(tag.size as usize, 8);
        unsafe {parse_boot_info_tag(tag);}

        if tag.type_id > 21 {panic!("Fuck");}
        if tag.type_id==0 && tag.size == 8 {break;}
        cur_pos += align_upper(tag.size as usize, 8);
    }
    serial_println!("Parsing ended.");
    Some(())
}




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

