use core::sync::atomic::{AtomicU64, Ordering};
use crate::alloc::boxed::Box;

mod stack;
pub use stack::Stack;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TaskId(u64);

#[repr(u8)]
pub enum TaskState {
    Created, 
    Waiting,
    Running,
    Zombie
}

impl TaskId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}


#[repr(C)]
/// Represent a TCB/context/Task
pub struct Task  {
    pub id: TaskId,

    pub cr3: usize,
    pub state: TaskState,
    pub stack: Stack
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

