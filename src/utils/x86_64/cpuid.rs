use core::str::from_utf8;
use crate::alloc::boxed::Box;



// The EDX flags are in the low bytes
// The ECX flags are in the high bytes
#[allow(non_camel_case_types)]
#[repr(u64)]
pub enum Features {
    // Those are all the flags in ECX
    SSE3         = 1 << (0  + 32),
    PCLMUL       = 1 << (1  + 32),
    DTES64       = 1 << (2  + 32),
    MONITOR      = 1 << (3  + 32), 
    DS_CPL       = 1 << (4  + 32), 
    VMX          = 1 << (5  + 32), 
    SMX          = 1 << (6  + 32), 
    EST          = 1 << (7  + 32), 
    TM2          = 1 << (8  + 32),  
    SSSE3        = 1 << (9  + 32),  
    CID          = 1 << (10 + 32),
    FMA          = 1 << (12 + 32),
    CX16         = 1 << (13 + 32), 
    ETPRD        = 1 << (14 + 32), 
    PDCM         = 1 << (15 + 32), 
    PCIDE        = 1 << (17 + 32), 
    DCA          = 1 << (18 + 32), 
    SSE4_1       = 1 << (19 + 32), 
    SSE4_2       = 1 << (20 + 32), 
    x2APIC       = 1 << (21 + 32), 
    MOVBE        = 1 << (22 + 32), 
    POPCNT       = 1 << (23 + 32), 
    AES          = 1 << (25 + 32), 
    XSAVE        = 1 << (26 + 32), 
    OSXSAVE      = 1 << (27 + 32), 
    AVX          = 1 << (28 + 32),
    // Those are the flags in EDX
    FPU          = 1 << 0,  
    VME          = 1 << 1,  
    DE           = 1 << 2,  
    PSE          = 1 << 3,  
    TSC          = 1 << 4,  
    MSR          = 1 << 5,  
    PAE          = 1 << 6,  
    MCE          = 1 << 7,  
    CX8          = 1 << 8,  
    APIC         = 1 << 9,  
    SEP          = 1 << 11, 
    MTRR         = 1 << 12, 
    PGE          = 1 << 13, 
    MCA          = 1 << 14, 
    CMOV         = 1 << 15, 
    PAT          = 1 << 16, 
    PSE36        = 1 << 17, 
    PSN          = 1 << 18, 
    CLF          = 1 << 19, 
    DTES         = 1 << 21, 
    ACPI         = 1 << 22, 
    MMX          = 1 << 23, 
    FXSR         = 1 << 24, 
    SSE          = 1 << 25, 
    SSE2         = 1 << 26, 
    SS           = 1 << 27, 
    HTT          = 1 << 28, 
    TM1          = 1 << 29, 
    IA64         = 1 << 30,
    PBE          = 1 << 31
}



pub unsafe fn  __cpuid(level: u32) -> [u32; 4]{
    let mut v = [0u32; 4];
    unsafe {
        asm!("cpuid", in("ax") level, 
        lateout("ax") v[0], lateout("bx") v[1], lateout("cx") v[2], lateout("dx") v[3]);
    }
    return v;
}


/**
 * Safety: expects the CPU to support CPUID
 */
pub fn cpu_vendor() -> Box<str> {

    unsafe {
        let v = __cpuid(0);
        // Thats not very nice, i know
        let v = [v[1],v[3],v[2]];
        let v: [u8;12] = core::mem::transmute(v);
        let s = from_utf8(&v).unwrap();
        return Box::from(s);
    }
}

/**
 * Safety: expects the CPU to support CPUID
 */
pub fn highest_leaf() -> u32 {
    return unsafe { __cpuid(0)[0] };
}


pub fn feature_set() -> (u32,u32) {
    let v = unsafe { __cpuid(1)};
    return (v[2],v[3]);
}

pub fn supports(f: Features) -> bool{
    let flag = {
        let (cx, dx) = feature_set() ;
        ((cx as u64) << 32) | (dx as u64)
    };

    return ((f as u64) & flag) != 0;
}


