
use crate::alloc::boxed::Box;

mod stack;
pub mod scheduler;
mod tasks;

use tasks::*;


pub use tasks::Task; 
pub use stack::Stack;



use spin::Mutex;
use core::cell::RefCell;
use crate::alloc::collections::BTreeMap;

pub (self) static TASK_MAP : Mutex<BTreeMap<TaskId, RefCell<Task>>> = Mutex::new(BTreeMap::new());

/// TaskId of the current running task
pub (self) static CURRENT_TASK: Mutex<TaskId> = Mutex::new(TaskId::default());



/// Set the current running task as a valid one 
/// Dropping this task will result in strange behavior
pub unsafe fn init_multitasking() -> TaskId {

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
    
    let id = cur_task.id;

    let res = (*TASK_MAP.lock()).insert(cur_task.id, RefCell::new(cur_task));
    assert!(res.is_none(), "Task already existed !");

    scheduler::init_scheduler(id);

    return id;
}




//TODO: move it somewhere else ?
const STACK_SIZE: usize = 4 * crate::utils::units::KiB;

pub unsafe fn create_task(func: fn()) -> TaskId {

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
    let id = task.id;

    let res = (*TASK_MAP.lock()).insert(task.id, RefCell::new(task));
    assert!(res.is_none(), "Task already existed !");

    scheduler::register_task(id);

    return id;


}
 


pub fn switch_task<'a>(next:TaskId){

    // Yes this function sucks.
    // But i'm struggling to pass those field ptrs without using the stack 

    // The current_task is invalid until the switch happens
    let cur = *CURRENT_TASK.lock();
    *CURRENT_TASK.lock() = next;


    let lock = TASK_MAP.lock();

    let cur_top_ptr;
    let next_top_ptr;
    {
        let tasks = &*lock;

        let cur = tasks.get(&cur).unwrap();
        let mut cur = cur.borrow_mut();
        cur_top_ptr = &mut cur.stack.top as *mut usize;
        drop(cur);

        let next = tasks.get(&next).expect("The scheduler provided an invalid taskid.");
        let mut next = next.borrow_mut();
        next_top_ptr = &mut next.stack.top as *mut usize;    
        drop(next);

    }
    // Make sur to drop this lock
    drop(lock);

    
    // Uncommenting this line produces a page fault when 
    // executing the second context  switch on the same task.
    // It turns out that this is because the future rip value
    // (0x0020fd1c) which is stored on the T1 stack 
    // (0x4444beef1dd0 for a 4096 stack at 0x4444beef2060)
    // get overriden by some random value (0xc). 
    // Is this is a stack overflow ?

    //crate::println!("cur:{:#x} next {:#x}", cur.stack.top as usize, next.stack.top as usize);
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {unsafe { 

        crate::arch::task_switch::context_switch(
            // This is a double level of indirection
            // that is, we need a pointer to the stack_top pointer
            cur_top_ptr as *mut u64,
            next_top_ptr as *mut u64);
     } 
    });

}

