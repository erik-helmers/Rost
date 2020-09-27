use crate::multitasking::{Task};

use crate::println;
/**
 * Switch the current TCB to the next TCB
 *
 * SAFETY: the function should be called with interrupt disabled
 */
pub unsafe extern "sysv64" fn switch_task(cur_task: &mut Task, next_task: &mut Task) {
    // Save previous task state on own stack :
    // https://en.wikipedia.org/wiki/X86_calling_conventions#System_V_AMD64_ABI
    // > if the callee wishes to use registers RBX, RBP, R12-R15 it must restore their 
    // > original values before returning control to the caller. All other registers 
    // > must be saved by the caller if it wishes to preserve their values
    asm!("
        push rbx    
        push rbp

        push r12
        push r13
        push r14
        push r15
        ");
    // We switch stack
    cur_task.stack.set_top_ptr(super::instructions::rsp() as *mut u8);
    super::instructions::set_rsp(next_task.stack.top_ptr() as usize);
        
    //println!("TopPtr {:#x}", cur_task.stack.top_ptr() as usize);
    // Restore registers from the new task 
    // Other registers were caller saved
    asm!("
        pop r15
        pop r14
        pop r13
        pop r12

        pop rbp
        pop rbx
        ret
    ");
}