use super::{Task, TaskId};
use alloc::{collections::BTreeMap, sync::Arc};
use core::task::Waker;
use crossbeam_queue::ArrayQueue;

use core::task::{Context, Poll};


pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task: Task){
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some(){
            panic!("Trying to spawn a task multiple times");
        }
        self.task_queue.push(task_id).expect("Task queue full!");

    }

    pub fn run_ready_tasks(&mut self){
        let Self {
            tasks,
            task_queue,
            waker_cache
        } = self;

        while let Ok(id) = task_queue.pop(){
            let task = match tasks.get_mut(&id){
                Some(task) => task,
                None => continue,
            };
            let waker = waker_cache
                .entry(id)
                .or_insert_with(|| TaskWaker::new(id, task_queue.clone()));
            
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context){
                Poll::Ready(()) => {
                    tasks.remove(&id);
                    waker_cache.remove(&id);
                }
                Poll::Pending => {}
            }
        }
    }

    pub fn run(&mut self) -> !{
        loop { 
            self.run_ready_tasks(); 
            self.sleep();
        }
    }

    fn sleep(&mut self){
        if self.task_queue.is_empty() {
            x86_64::instructions::hlt();
        }
    }
}



struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}


impl TaskWaker {
    fn new(id:TaskId, tq: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(Self {task_id:id, task_queue:tq}))
    }
    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

use alloc::task::Wake;

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}
