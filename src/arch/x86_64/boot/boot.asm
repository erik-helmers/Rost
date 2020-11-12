section .multiboot_header

header_start:
    dd 0xe85250d6                ; magic number (multiboot 2)
    dd 0                         ; architecture 0 (protected mode i386)
    dd header_end - header_start ; header length
    ; checksum -(magic + header_flag + size)
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; insert optional multiboot tags here

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd  8    ; size
header_end:
section .inittext
global _init
bits 32

extern boot_stack_top
extern boot_info_addr
extern p4
extern p3

; This is the function called by GRUB
; We are in 32bit, no paging 
_init:

    mov dword [0xb8000], 0x2f4b2f4f

    ; I don't really understand why lea esp, stack_top
    ; works whereas mov gives invalid result 
    ;  -> Its because the correct syntax is `mov esp, offset boot_stack_top`
    lea esp, boot_stack_top
    mov [boot_info_addr], ebx  
    call check_multiboot

    call check_cpuid
    call check_long_mode

    ; We maually clear every page
    mov eax, p4
    call clear_page
    mov eax, p3
    call clear_page
    call init_ident_pages
    call enable_paging
            

    lgdt [gdt64.pointer]
    
    mov dword [0xb8000], 0x2f4f2f4b
    
    jmp gdt64.code:_init64
    
check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, '0'
    jmp error

check_cpuid:
    ; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
    ; in the FLAGS register. If we can flip it, CPUID is available.

    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the
    ; ID bit back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit
    ; wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, '1'
    jmp error


check_long_mode:
    ; test if extended processor info in available
    mov eax, 0x80000000    ; implicit argument for cpuid
    cpuid                  ; get highest supported argument
    cmp eax, 0x80000001    ; it needs to be at least 0x80000001
    jb .no_long_mode       ; if it's less, the CPU is too old for long mode

    ; use extended info to test if long mode is available
    mov eax, 0x80000001    ; argument for extended processor info
    cpuid                  ; returns various feature bits in ecx and edx
    test edx, 1 << 29      ; test if the LM-bit is set in the D-register
    jz .no_long_mode       ; If it's not set, there is no long mode
    ret
.no_long_mode:
    mov al, '2'
    jmp error

; Zeroes a page 
; Expect a pointer to page 
; Clobs ECX
clear_page:
    mov ecx, 0 
.loop:
    mov dword  [eax+ecx*4], 0x00000000
    add ecx, 1
    cmp ecx, 0x400
    jb .loop
.end:
    ret
    


; Setup identity paging up to 4GiB
init_ident_pages: 
    mov eax, p3
    or  eax, 0b11
    mov [p4], eax
    
    mov dword [p3],    0x83
    mov dword [p3+8],  0x40000083
    mov dword [p3+16], 0x80000083
    mov dword [p3+24], 0xC0000083
    ret



enable_paging:
    ; https:;wiki.osdev.org/CPU_Registers_x86-64#Control_Registers
    ; load P4 to cr3 register (cpu uses this to access the P4 table)
    mov eax, p4
    mov cr3, eax
    
    ; enable PAE-flag in cr4 (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    ret

; Prints `ERR: ` and the given error code to screen and hangs.
; parameter: error code (in ascii) in al
error:
    mov dword  [0xb8000], 0x4f524f45
    mov dword  [0xb8004], 0x4f3a4f52
    mov dword  [0xb8008], 0x4f204f20
    mov byte   [0xb800a], al
    hlt


section .inittext
bits 64 

_init64:
    extern _start

    ; We clear data semgent registers 
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    ; print `OKAY` to screen
    mov rax, 0x2f592f412f4b2f4f
    mov qword  [0xb8000], rax
    call init_hh_pages
    ; We now have higher half paging, which means
    ; we can use the stack as we want
    mov rsp, kstack_top
    
    mov  edi, [boot_info_addr]
    call _start
    hlt 

; Init higher half pages 
; Identity maps phys 2GiB to virt -2GiB
;
; TODO: this will break if we change the mapping
; Because 0xffff_ffff_8000_000 is hardcoded in a 
; a non trivial way
init_hh_pages:
    mov eax,  p3
    or eax, 0b11
    mov [p4+511*8], eax

    mov dword [p3+510*8], 0x00000083
    mov dword [p3+511*8], 0x40000083
    ; we dont forget to flush our changes
    mov rax, cr3
    mov cr3, rax
    ret 

gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
; Credits to elyalyssamathys

section .bss
; Page aligned section
kstack_bottom: 
    resb 0x4000
kstack_top: 
