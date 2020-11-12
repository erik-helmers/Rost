pub static GDT: [u64; 2] = [
    0,
    // code segment
    (1<<43) | (1<<44) | (1<<47) | (1<<53)
];

#[repr(C, packed)]
struct Loader {
    size: u16,
    addr: usize
}

pub fn init_gdt(){
    let loader = Loader {
        size: core::mem::size_of::<u64>() as u16*2,
        addr: &GDT as *const _ as usize
    };
    unsafe {
        crate::arch::instructions::lgdt(&loader as *const _ as  u64);
    }


}
