.section .field-test, "awx"
.global push_64_pop_8
.global sw_stack_pop
.intel_syntax noprefix
.code64

push_64_pop_8:
    mov rax, 0
    push rdi
    pop dx
    pop dx
    pop dx
    pop ax


    ret
sw_stack_pop:
    mov rcx, rsp // store old stack ptr 
    mov rsp, rdi // switch stack
    pop rax      // pop value 
    mov rsp, rcx // switch back 
    ret

get_cr3:
    mov rax, [rdi+8]
    ret