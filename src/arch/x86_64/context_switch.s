.section .context-switch, "awx"
.global context_switch
.intel_syntax noprefix
.code64

// Context switch, SYSV64 calling convention
// void context_switch(uint_64 &cur_rsp, uint_64 &next_rsp)  
// Safety: 
// Do not call while holding locks or with interrupts enabled
context_switch:

    push rbx    
    push rbp

    push r12
    push r13
    push r14
    push r15

    
    mov [rdi], rsp
    mov rsp, [rsi]

        

    pop r15
    pop r14
    pop r13
    pop r12

    pop rbp
    pop rbx
    
    ret
