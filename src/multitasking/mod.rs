use core::sync::atomic::{AtomicU64, Ordering};



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TaskId(u64);

impl TaskId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
#[repr(u8)]
pub enum TaskState {
    Created, 
    Waiting,
    Running,
    Zombie
}

#[repr(C)]
/// Represent a TCB/context/Task
pub struct Task  {
    pub id: TaskId,

    pub cr3: usize,
    pub state: TaskState,
    pub stack: Stack
}

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
        let mut array:Box<[u8]> = Box::from(vec![0u8; STACK_SIZE]) ;
        let top = unsafe{array.as_mut_ptr().add(size)};
        Self{array, top }
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



/// Set the current running task as a valid one 
/// Dropping this task will result in strange behavior
pub unsafe fn init_multitasking() -> Box<Task> {

    let stack  = Stack::new_zero_sized();

    let cur_task = Task {
        id: TaskId::new(),
        // This is a hack: we didn't initialize the stack here
        // And we don't expect to deallocate it so we just 
        // force our way 
        stack,
        //FIXME: 
        cr3:0,
        state: TaskState::Running,
    };
    Box::new(cur_task)

}




//TODO: move it somewhere else ?
const STACK_SIZE: usize = 4 * crate::utils::units::KiB;


pub unsafe fn create_task(func: fn()) -> Box<Task>{

    let mut stack = Stack::new(STACK_SIZE);
    
    
    // We "populate" the stack with dummy values 
    unsafe { 
        // The RIP should be first
        stack.push_u64(func as *const () as u64);
        // rbp + rbx + r12 - r15 = 6*64 bits = 6*8 byte
        for _ in &[0u8; 6*8] { stack.push(0); }
    };

    //We need to allocate 
    let task = Task {
        id: TaskId::new(),
        stack,
        cr3: 0,
        state: TaskState::Created,
    };
    Box::new(task)

}
 


pub fn switch_task<'a>(cur:&mut Task, next:&mut Task){

    use x86_64::instructions::interrupts;
    //crate::println!("{:#x}", next.stack.top_ptr() as usize);
    interrupts::without_interrupts(|| {unsafe { 

        crate::arch::task_switch::switch_task(cur, next);
     }
    });

}


#[test_case]
pub fn valid_stack_top(){
    let stack = Stack::new(STACK_SIZE);
    let start = stack.array.first().expect("Zero sized ?!") as *const u8 as usize;
    let end = stack.top as usize;

    assert!(start < end);
     
}
