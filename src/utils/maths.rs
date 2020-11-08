crate::import_commons!();

/// Pads to the nearest upper aligned address
/// The pad should be a power of two
/// ```
/// assert_eq!(align_upper(15, 8), 16);
/// assert_eq!(align_upper(15, 16), 16);
/// assert_eq!(align_upper(26, 8), 32); 
/// ```
pub fn align_upper(num: usize, pad:usize) -> usize{
    if num & (pad-1) == 0 { num }
    else { (num & (!(pad-1))) + pad  }
}

#[cfg(test)]
#[test_case]
pub fn align_correct(){
    assert_eq!(align_upper(15, 8), 16);
    assert_eq!(align_upper(15, 16), 16);
    assert_eq!(align_upper(26, 8), 32);
}