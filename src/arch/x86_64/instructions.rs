

// Here we implement i/o instruction, namely in(x) / out(x) commands and sti, cli
// With the 3 possible sizes, byte, word, double word
// See:
// https://doc.rust-lang.org/beta/unstable-book/library-features/llvm-asm.html
// https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html
// https://hjlebbink.github.io/x86doc/html/IN.html
// https://hjlebbink.github.io/x86doc/html/OUT.html

pub unsafe fn sti(){
    asm!("sti",  options(nostack, nomem));
}

pub unsafe fn cli(){
    asm!("cli", options(nostack, nomem));
}

    
// 8 bit 

/// Write a byte to specified port
pub unsafe fn outb(port:u16, data: u8){
    asm!( "out dx, al", in("dx") port, in("al") data);
}


/// Reads a byte from specified port
pub unsafe fn inb(port:u16) -> u8 {
    let ret :u8;
    asm!( "in al, dx", in("dx") port, lateout("al") ret);
    ret
}

// 16 bit 

/// Writes a word to specified port
pub unsafe fn outw(port: u16, data: u16) {
    asm!( "out dx, ax", in("dx") port, in("ax") data);
}

/// Reads a word from specified port
pub unsafe fn inw(port:u16) -> u16 {
    let ret: u16;
    asm!( "in ax, dx", in("dx") port, lateout("ax") ret);
    ret
}

// 32 bit 

/// Writes a double word to specified port
pub unsafe fn outdw(port: u16, data: u32) {
    asm!( "out dx, eax", in("dx") port, in("eax") data);
}

/// Reads a double word from specified port
pub unsafe fn indw(port:u16) -> u32 {
    let ret: u32;
    asm!( "in eax, dx", in("dx") port, lateout("eax") ret);
    ret
}

// Register reading
/// Read ESP 
pub unsafe fn rsp() -> usize {
    let ret: u64;
    asm!("mov {}, rsp", out(reg) ret);
    ret as usize 
}

// Here we really need the inline(always) 
// because we are changing the stacktop address
#[inline(always)]
/// Set ESP 
pub unsafe fn set_rsp(rsp: usize) {
    asm!("mov rsp, {}", in(reg) rsp);
}