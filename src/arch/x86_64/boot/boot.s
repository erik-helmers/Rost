.section .inittext, "awx"
.global _init
.intel_syntax noprefix
.code32

// This is the function called by GRUB
// We are in 32bit, no paging 
_init:
    // print `OK` to screen

    mov dword PTR [0xb8000], 0x2f4b2f4f
    hlt

