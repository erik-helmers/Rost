//! In this module we try to represent fixed point numbers
//! As simply as possible.
//! 
//! TODO:
//!     - math ops
//!     - display
//!     - generic support

use core::ops::*;
use core::fmt::Display;

/// Basic trait for representing a number
/// Please note however that is not strict enough
pub trait Integer<T>: Add<T> + Sub<T> + Mul<T> + Div<T> + Copy + From<u32> {}


#[derive(Copy, Clone)]
pub struct FixedPoint32_32 {
    val: u64    
}

impl FixedPoint32_32 {
    pub fn new() -> Self { Self { val: 0 }}
    pub fn int(&self) -> u32 { (self.val >> 32) as u32}
    pub fn frac(&self) -> u32 { self.val as u32 }
}   

impl From<(u32,u32)> for FixedPoint32_32 {
    fn from((int, frac): (u32,u32)) -> Self {
        Self {val: (int as u64) << 32 | frac as u64 }
    }
}

//TODO: impl From<f64> ? 

impl Display for FixedPoint32_32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        //TODO: how to implement fast and accurate decimal representation without floats?
        let mut frac = self.frac() as f64 / u32::MAX as f64 ;
        frac *= 1_000_000_000 as f64;
        write!(f, "{}.{:09.0}", (self.val >> 32) as u32, frac)
    }
    
}
impl Add<FixedPoint32_32> for FixedPoint32_32 {
    type Output = Self;
    fn add(self, rhs: FixedPoint32_32) -> Self::Output {
        Self { val: self.val + rhs.val }
    }
}

impl Sub<FixedPoint32_32> for FixedPoint32_32 {
    type Output = Self;
    fn sub(self, rhs: FixedPoint32_32) -> Self::Output {
        Self { val: self.val - rhs.val }
    }
}

impl Mul<u32> for FixedPoint32_32 {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self { val: self.val * rhs as u64 }
    }    
}

impl Div<u32> for FixedPoint32_32 {
    type Output = Self;
    fn div(self, rhs: u32) -> Self::Output {
        Self { val: self.val / rhs as u64}
    }
}






/* pub struct FixedPoint<T: Integer<T>> {
    int: T,
    frac: T
}

impl<T:Integer<T>> FixedPoint<T> {
    pub fn new() -> Self {
        FixedPoint { int: T::from(0), frac: T::from(0) }
    }
}

impl<T:Integer<T>> Add<T> for FixedPoint<T> {
    type Output = FixedPoint<T>;
    fn add(self, rhs: T) -> Self::Output {
        
    }   
} */