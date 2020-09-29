
/// This represents a kernel stack 
pub struct Stack {
    pub array: Box<[u8]>,
    /// Stores the index of the top (bottom) of the stack.
    /// This an offset to the base array 
    top: *mut u8 
}


use crate::alloc::vec;
use crate::alloc::boxed::Box;


impl Stack {
    /// Creates a new non zero sized stack 
    pub fn new(size: usize) -> Self {
        // We allocate a new stack
        let mut array:Box<[u8]> = Box::from(vec![0u8; size]) ;
        let top = unsafe{array.as_mut_ptr().add(size)};
        Self{array, top}
    }


    pub fn set_top_ptr(&mut self, top: *mut u8){
        self.top = top;
    } 

    pub fn top_ptr(&self) -> *const u8 {
        self.top
    }

    pub fn top_ptr_mut(&self) -> *mut u8 {
        self.top
    }
    /// This method is only called when emulating an already existing task 
    /// Which should never be dropped
    pub fn new_zero_sized() -> Self {
        Self{array: Box::new([0u8;0]), top: 0 as *mut u8}
    }

    /// Push a 8 bit value to the stack, growing downwards
    pub unsafe fn push(&mut self, val: u8) {
        self.top = self.top.offset(-1);
        *self.top = val;
    }
    
    pub unsafe fn push_u64(&mut self, val: u64){
        self.top = self.top.offset(-8);
        // Yolo 
        *(self.top as *mut u64) = val;
    }

    /// Pop a value from the stack
    pub unsafe fn pop(&mut self) {
        self.top = self.top.offset(1);
    }

}

