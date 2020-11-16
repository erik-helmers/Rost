use crate::common::memory as mem;
use mem::*;
use alloc::FrameAllocator;
use paging::*;

pub fn kernel_remap<A>(rapt4: &mut ActivePageTable, alloc: &mut A) where A: FrameAllocator{
    rapt4.create_and_set_new_p4(209, alloc, |mapper, alloc|{
        
        let addr1 = VirtAddr::new(0xffff_ffff_8000_0000);
        let addr2 = VirtAddr::new(0);

        let frame = Frame::new(PhysAddr::new(0), SizeType::HugeP3);
        mapper.map_to(Page::new(addr1, SizeType::HugeP3), frame.clone(), 
            PDF::PRESENT|PDF::WRITABLE, alloc);

        mapper.map_to(Page::new(addr2, SizeType::HugeP3), frame, 
            PDF::PRESENT|PDF::WRITABLE, alloc);
        
        
    });
}
