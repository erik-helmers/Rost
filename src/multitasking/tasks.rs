use core::{fmt::Debug, sync::atomic::{AtomicU64, Ordering}};
use super::stack::Stack;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
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

/// This task number is never used
impl Default for TaskId {
    fn default() -> Self {
        Self(0)
    }
}

impl TaskId {
    pub const fn default() -> Self {
        Self(0)
    }
    pub const fn from_int(val:u64) -> Self {
        Self(val)
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


impl Debug for Task {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Task")
         .field("id", &self.id)
         .field("stack [start-top]", &self.stack.top_ptr())
         .finish()
    }
}

