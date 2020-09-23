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


#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]

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
        // TODO: figure out why using println! causes a deadlock when calling write!
        // voir cours de PF1, méthode de multiplications successives
        // conversion de 0.{frac base 2} en base 2 vers {frac base 10}
        let mut acc = FixedPoint32_32::from((0,self.frac()));
        let mut frac = 0u32;
        let mut count = 0;

        while acc.frac() != 0 && count <= 10 {
            acc *= 10;
            frac = frac*10 + acc.int();   
            //TODO: not clean
            acc.val &= u32::MAX as u64;
            count+=1;
        }

        write!(f, "{}.{:0width$}", (self.val >> 32) as u32, frac, width=count)
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

impl MulAssign<u32> for FixedPoint32_32 {
    fn mul_assign(&mut self, rhs: u32) {
        self.val *= rhs as u64;
    }
}

impl Div<u32> for FixedPoint32_32 {
    type Output = Self;
    fn div(self, rhs: u32) -> Self::Output {
        Self { val: self.val / rhs as u64}
    }
}


#[test_case]
pub fn addition_works(){
    let a = FixedPoint32_32::from((0,1));
    let b =FixedPoint32_32::from((0,2));
    let c = FixedPoint32_32::from((0,3));
    assert_eq!(a+b, c);
}

#[test_case]
pub fn fmt_works(){
    use crate::alloc::string::ToString;

    let a = FixedPoint32_32::from((0,0b011 << 29));
    let b = FixedPoint32_32::from((0,0b1011 << 28));

    assert_eq!("0.375", a.to_string());  
    assert_eq!("0.6875", b.to_string());
    
    
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