use crate::bitflags;

use super::memory::VirtAddr;


 /// http://www.skyfree.org/linux/references/ELF_Format.pdf
 #[derive(Debug, Copy, Clone, Eq, PartialEq)]
 #[repr(u32)]
 pub enum SectionType {
    NULL = 0,
    PROGBITS = 1,
    SYMTAB = 2,
    STRTAB = 3,
    RELA = 4,
    HASH = 5,
    DYNAMIC = 6,
    NOTE = 7,
    NOBITS =  8,
    REL = 9,
    SHLIB = 10,
    DYNSYM = 11,

    LOPROC = 0x70000000,
    HIPROC = 0x7fffffff,
    LOUSER = 0x80000000,
    HIUSER = 0xffffffff
}


bitflags!{
    pub struct SectionHeaderFlags(u64){
        /// Writable
        const WRITE = 0x1;
        /// Occupies memory during execution
        const ALLOC = 0x2;
        /// Executable
        const EXECINSTR = 0x4;
        /// Might be merged
        const MERGE = 0x10;

        /// Contains null terminated strings
        const STRINGS = 0x20;

        /// 'sh_info' contains SHT index
        const SHF_INFO_LINK = 0x40;
        
        /// Preserve order after combining
        const LINK_ORDER = 0x80;

        ///Section is member of a group
        const GROUP = 0x200;
        ///Section hold thread-local data
        const TLS = 0x400;
        ///OS-specific
        const MASKOS = 0x0ff00000;
        ///Processor-specific
        const MASKPROC = 0xf0000000;
    }    
}

#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct SectionHeader64 {
    /// An offset to a string in the .shstrtab section that represents the name of this section. 
    pub sh_name: B<u32>,
    ////Identifies the type of this header. 
    pub sh_type: SectionType,
    /// Identifies the attributes of the section. 
    pub sh_flags:SectionHeaderFlags,
    /// Virtual address of the section in memory, for sections that are loaded. 
    pub sh_addr: VirtAddr,
    /// Offset of the section in the file image. 
    pub sh_offset: B<u64>,
    ///Size in bytes of the section in the file image. May be 0. 
    pub sh_size: B<u64>,
    /// Contains the section index of an associated section. This field is used
    /// for several purposes, depending on the type of section. 
    pub sh_link: u32,
    ///Contains extra information about the section. This field is used for 
    /// several purposes, depending on the type of section. 
    pub sh_info: B<u32>,
    ///Contains the required alignment of the section. This field must be a power of two. 
    pub sh_addralign: B<u64>,
    ///Contains the size, in bytes, of each entry, for sections that contain
    /// fixed-size entries. Otherwise, this field contains zero. 
    pub sh_entsize: B<u64>,
}   
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SectionHeader32 {
    /// An offset to a string in the .shstrtab section that represents the name of this section. 
    sh_name: B<u32>,
    ////Identifies the type of this header. 
    sh_type: B<u32>,
    /// Identifies the attributes of the section. 
    sh_flags:B<u32>,
    /// Virtual address of the section in memory, for sections that are loaded. 
    sh_addr: B<u32>,
    /// Offset of the section in the file image. 
    sh_offset: B<u32>,
    ///Size in bytes of the section in the file image. May be 0. 
    sh_size: B<u32>,
    ///Contains the section index of an associated section. This field is used for several purposes, depending on the type of section. 
    sh_link: B<u32>,
    ///Contains extra information about the section. This field is used for several purposes, depending on the type of section. 
    sh_info: B<u32>,
    ///Contains the required alignment of the section. This field must be a power of two. 
    sh_addralign: B<u32>,
    ///Contains the size, in bytes, of each entry, for sections that contain fixed-size entries. Otherwise, this field contains zero. 
    sh_entsize: B<u32>,
}





#[repr(transparent)]
#[derive(Copy, Clone)]

/// FIXME: that struct is a fast patch 
/// should be replaced by VirtAddr and PhysAddr
pub struct B<T>(T);

impl core::fmt::Debug for B<u32> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("{:#x}", self.0))
    }
}           

impl core::fmt::Debug for B<u64> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_fmt(format_args!("{:#x}", self.0))
    }
}
