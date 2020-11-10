
#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test::runner)]
#![reexport_test_harness_main = "test_main"]


#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![allow(unused_unsafe)]


use core::cmp::min;

use rost_nbs::*;
import_commons!();

use devices::multiboot2::*;

#[no_mangle]
pub unsafe extern "sysv64" fn _start(_boot_info: *const ()) {

    // Dumping all tags generic
    

    let mbi = MultibootInfo::new(_boot_info);

    dump_mbi(&mbi);
    serial_println!("");

    let mmap = mbi.find::<MemoryMap>();
    if mmap.is_some() {dump_mmap(mmap.unwrap());}
    else {serial_println!("Memory map not found.");}
    serial_println!("");

    let elf = mbi.find::<ELFSymbols>();
    if elf.is_some() {dump_elf(elf.unwrap());}
    else {serial_println!("ELF symbols not found.");}
    

    devices::qemu_debug::exit(devices::qemu_debug::Status::Success);
}


pub unsafe fn dump_mbi(mbi: &MultibootInfo){
    
    serial_println!("MBI struct: total_size: {0} = {0:#x}", mbi.total_size());

    for (i, tag) in mbi.into_iter().enumerate() {
        serial_print!("Tag #{}: ", i);
        dump_tag(mbi, tag.as_ref().unwrap());
    }
}

unsafe fn dump_tag(mbi: &MultibootInfo, tag: &TagHeader) -> Option<()>{
    let ptr = tag as *const TagHeader;
    let content : &dyn core::fmt::Debug = match tag.type_id {
        4 => ptr.cast::<BasicMemoryInformation>().as_ref()?,
        5 => ptr.cast::<BIOSBootDevice>().as_ref()?,
        1 => {
            let val = ptr.cast::<BootCommandLine>().as_ref()?;
           serial_println!("{:#?}: '{}'", val, val.string());
           return None;
        }
        3 => ptr.cast::<Modules>().as_ref()?,
        9 => ptr.cast::<ELFSymbols>().as_ref()?,
        6 => ptr.cast::<MemoryMap>().as_ref()?,
        2 => { 
            let val = ptr.cast::<BootLoaderName>().as_ref()?;
            serial_println!("{:#?}: '{}'", val, val.string());
            return None;
        },
       10 => ptr.cast::<APMTable>().as_ref()?,
        7 => ptr.cast::<VBEInfo>().as_ref()?,
        8 => ptr.cast::<FrameBufferInfo>().as_ref()?,
       21 => ptr.cast::<ImageLoadBasePhysicalAddress>().as_ref()?,


       _ => tag,
   };
   serial_println!("{:#?}", content);

   None

}
    

pub fn dump_mmap(mmap: &MemoryMap){
    serial_println!("Memory map: {} entries", mmap.nb_entries());
    for i in 0..mmap.nb_entries(){
        serial_println!("Entry #{}: {:?}", i, mmap[i]);
    }
}

pub unsafe fn dump_elf(elf: &ELFSymbols){
    serial_println!("ELF symbols : {} sections", elf.num);

    let num = min(10, elf.num);

    for i in 0..num {
        let sh = elf.at(i as _ ).unwrap();
        serial_println!("Section #{}: {:#?}", i, sh);    
    }
    
    if num < elf.num { serial_println!("... Omitted {} sections.", elf.num - num); }
}
    

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler(info);
}