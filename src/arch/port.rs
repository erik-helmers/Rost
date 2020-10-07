
use crate::utils::x86_64::instructions;
use core::marker::PhantomData;

pub trait Read { unsafe fn read(port: u16) -> Self; }
pub trait Write { unsafe fn write(port: u16, data: Self);}

// Please notice that those impl just points to the
// corresponding asm instruction defined in arch::instruction::in/out 
impl Read for u8 {
    unsafe fn read(port: u16) -> Self { instructions::inb(port) } }
impl Write for u8 {
    unsafe fn write(port: u16, data: Self) { instructions::outb(port, data); }}
impl Read for u16 {
    unsafe fn read(port: u16) -> Self { instructions::inw(port) } }
impl Write for u16 {
    unsafe fn write(port: u16, data: Self) { instructions::outw(port, data); }}
impl Read for u32 {
    unsafe fn read(port: u16) -> Self { instructions::indw(port) } }
impl Write for u32 {
    unsafe fn write(port: u16, data: Self) {instructions::outdw(port, data);}}

/// Represents a n bit port
pub struct Port<T: Read+Write> {
    port: u16,
    size: PhantomData<T>
}

impl<T:Read+Write> Port<T> {
    pub const fn new(port: u16) -> Self {
        Self { port, size: PhantomData }
    }
    pub unsafe fn write(&self, data: T) {
        T::write(self.port, data);
    }
    pub unsafe fn read(&self) -> T {
        T::read(self.port)
    }
}

// Those following structs are just boilerplate to
// allow for  write or read only ports

/// Represents a n bit readonly port
pub struct ReadonlyPort<T: Read> {
    port: u16,
    size: PhantomData<T>
}

impl<T:Read> ReadonlyPort<T> {
    pub const fn new(port: u16) -> Self {
        Self { port, size: PhantomData }
    }
    pub unsafe fn read(&self) -> T {
        T::read(self.port)
    }
}


/// Represents a n bit write only port
pub struct WriteonlyPort<T: Write> {
    port: u16,
    size: PhantomData<T>
}

impl<T:Write> WriteonlyPort<T> {
    pub const fn new(port: u16) -> Self {
        Self { port, size: PhantomData }
    }
    pub unsafe fn write(&self, data: T) {
        T::write(self.port, data);
    }
}
