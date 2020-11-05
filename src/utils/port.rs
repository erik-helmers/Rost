
use core::marker::PhantomData;

pub struct Port<T: PortSize> {
    num: u16,
    _psize: PhantomData<T>
}

pub trait PortSize {
    unsafe fn in_(port: u16) -> Self;
    unsafe fn out(port: u16, val: Self);
}


use crate::arch::instructions as instrux;

impl<T:PortSize> Port<T> {
    // Create a port with the specified number
    pub const fn new(port: u16) -> Port<T> {
        Port{num:port, _psize: PhantomData}
    }

    /// Reads a value from port
    /// Safety:
    /// This function may or may not have side effects.
    /// Depending on the port written to.
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe{T::in_(self.num)}
    }

    /// Reads a value from port
    /// Safety:
    /// The output may or may not be garbage.
    /// But calling this function won't corrupt memory
    #[inline(always)]
    pub fn write(&self, val: T) {
        unsafe{T::out(self.num, val)}
    }
    
}


// This is just a bind for each type of in_/out to the appropriate asm instructions
impl PortSize for u8 {
    #[inline(always)] unsafe fn in_(port: u16) -> Self    { instrux::inb(port) }
    #[inline(always)] unsafe fn out(port: u16, val: Self) { instrux::outb(port, val) }
}

impl PortSize for u16 {
    #[inline(always)] unsafe fn in_(port: u16) -> Self { instrux::inw(port) }
    #[inline(always)] unsafe fn out(port: u16, val: Self) { instrux::outw(port, val) }
}

impl PortSize for u32 {
    #[inline(always)] unsafe fn in_(port: u16) -> Self { instrux::indw(port) }
    #[inline(always)] unsafe fn out(port: u16, val: Self) { instrux::outdw(port, val) }
}
