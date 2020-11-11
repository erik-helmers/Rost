#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SectionHeader64 {
    /// An offset to a string in the .shstrtab section that represents the name of this section. 
    sh_name: B<u32>,
    ////Identifies the type of this header. 
    sh_type: B<u32>,
    /// Identifies the attributes of the section. 
    sh_flags:B<u64>,
    /// Virtual address of the section in memory, for sections that are loaded. 
    sh_addr: B<u64>,
    /// Offset of the section in the file image. 
    sh_offset: B<u64>,
    ///Size in bytes of the section in the file image. May be 0. 
    sh_size: B<u64>,
    ///Contains the section index of an associated section. This field is used for several purposes, depending on the type of section. 
    sh_link: B<u32>,
    ///Contains extra information about the section. This field is used for several purposes, depending on the type of section. 
    sh_info: B<u32>,
    ///Contains the required alignment of the section. This field must be a power of two. 
    sh_addralign: B<u64>,
    ///Contains the size, in bytes, of each entry, for sections that contain fixed-size entries. Otherwise, this field contains zero. 
    sh_entsize: B<u64>,
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
struct B<T>(T);

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
