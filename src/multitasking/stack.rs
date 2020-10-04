
/// This represents a kernel stack 
pub struct Stack {
    pub array: Box<[u8]>,
    pub top: usize
}


use crate::alloc::vec;
use crate::alloc::boxed::Box;


impl Stack {
    /// Creates a new non zero sized stack 
    pub fn new(size: usize) -> Self {
        // We allocate a new stack
        let mut array:Box<[u8]> = Box::from(vec![0xBDu8; size]) ;
        let top = unsafe{array.as_mut_ptr().add(size) as usize};
        Self{array, top}
    }


    pub fn top_ptr(&self) -> *const u8 {
        self.top as *const u8
    }

    /// Returns {self.top} as a *mut u8
    ///
    /// Safety: there is no guarentee that the pointer is valid
    /// so use and deref with great care
    pub fn top_ptr_mut(&self) -> *mut u8 {
        self.top as *mut u8
    }
    /// This method is only called when emulating an already existing task 
    /// Which should never be dropped
    pub fn new_zero_sized() -> Self {
        Self{array: Box::new([0u8;0]), top: 0 }
    }

    /// Push a 8 bit value to the stack, growing downwards
    pub unsafe fn push(&mut self, val: u8) {
        self.top -= 1;
        *self.top_ptr_mut() = val;
    }
    
    pub unsafe fn push_u64(&mut self, val: u64){
        self.top -= 8;
        // Yolo 
        *(self.top as *mut u64) = val;
    }

    /// Pop a value from the stack
    pub unsafe fn pop(&mut self) {
        self.top += 1;
    }

}

