crate::import_commons!();


/// Pads to the nearest upper aligned address
/// The pad should be a power of two
/// ```
/// assert_eq!(align_upper(15, 8), 16);
/// assert_eq!(align_upper(15, 16), 16);
/// assert_eq!(align_upper(26, 8), 32); 
/// ```
#[inline]
pub const fn align_upper(num: usize, pad:usize) -> usize{
    if align_lower(num, pad) == num {num}
    else {align_lower(num, pad) + pad}
}



/// Pads to the nearest lower aligned address
/// The pad should be a power of two
/// ```
/// assert_eq!(align_upper(15, 8), 16);
/// assert_eq!(align_upper(15, 16), 16);
/// assert_eq!(align_upper(26, 8), 32); 
/// ```
#[inline(always)]
pub const fn align_lower(num: usize, pad:usize) -> usize{
    num & (pad-1)
}


#[cfg(test)]
#[test_case]
pub fn align_correct(){
    assert_eq!(align_upper(15, 8), 16);
    assert_eq!(align_upper(15, 16), 16);
    assert_eq!(align_upper(26, 8), 32);
}
