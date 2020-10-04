///! First draft at scheduling
///!
///! We use round robbin schedduling 
///! To keep things simple for the moment

use crate::alloc::collections::VecDeque;
use super::tasks::*;
use super::CURRENT_TASK;

use spin::Mutex;
use lazy_static::lazy_static;

lazy_static!{
    /// This is a queue of the tasks to run
    static ref RUNNING_TASKS: Mutex<VecDeque<TaskId>> = Mutex::new(VecDeque::new());
}


/// Selects a task to run and switches to it.
pub fn schedule() {
    let next = select_task();
    super::switch_task(next);
}

pub fn select_task() -> TaskId {
    let mut tasks = RUNNING_TASKS.lock();

    // Add the current task to queue
    let cur = *CURRENT_TASK.lock();
    tasks.push_back(cur); 
    // Return first task in queue
    tasks.pop_front().unwrap()
}   


pub fn init_scheduler(task: TaskId){
    *CURRENT_TASK.lock() = task;
}

pub fn register_task(id: TaskId){
    (*RUNNING_TASKS.lock()).push_back(id);
}