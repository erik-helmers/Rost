
#![no_std]
#![no_main]


#![feature(custom_test_frameworks)]
#![test_runner(rost_nbs::test::runner)]
#![reexport_test_harness_main = "test_main"]


#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![allow(unused_unsafe)]


use rost_nbs::*;
import_commons!();

use devices::multiboot2::*;

#[no_mangle]
pub unsafe extern "sysv64" fn _start(_boot_info: *const ()) {

    let mbi = MultibootInfo::new(_boot_info);

    for (i,tag) in (&mbi).into_iter().enumerate() {
        let tag = tag.as_ref().unwrap();
        serial_print!("Tag #{}: ", i);

        dump_tag(&mbi, tag);
    }
    let mmap = mbi.find::<MemoryMap>().unwrap();
    serial_println!("{:?}", mmap);
    for i in 0..mmap.nb_entries(){
        serial_println!("Entry #{}: {:?}", i, mmap[i]);
    }

    devices::qemu_debug::exit(devices::qemu_debug::Status::Success);
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
    
    
    

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_handler(info);
}