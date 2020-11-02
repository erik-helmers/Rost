ENTRY(_init)
OUTPUT_FORMAT(elf64-x86-64)


KERNEL_BASE = 0xFFFFFFFF80000000;

SECTIONS {
    
    /* This is a standard offset for a kernel because there's
       often special memory under this limit (e.g. VGA memory)
    */
    . = 1M;

    /* https://www.math.utah.edu/docs/info/ld_3.html */
    /* > Return the size in bytes of the output file's headers.
       > You can use this number as the start address of the first
       > section, if you choose, to facilitate paging. 
    */

    . += SIZEOF_HEADERS;


    . = ALIGN(0x1000);
    page_table_start = .;
    p4 = .;
    . += 0x1000;
    p3 = .;
    . += 0x1000;
    p2 = .;
    . += 0x1000;
    p1 = .;
    . += 0x1000;
    page_table_end = .;


	.init : AT(ADDR(.init)) {
		KEEP( *(.multiboot-header) )
		*(.inittext)
		boot_stack_bottom = .;
		. += 64;
		boot_stack_top = .;
	}

	INIT_END = .;

	. += KERNEL_BASE;
	
	.text ALIGN(0x1000) : AT(ADDR(.text) - KERNEL_BASE) {
		*(.text .text.*)
	}

	/* read-only data, page aligned to allow use of the no-execute feature */
	.rodata ALIGN(0x1000) : AT(ADDR(.rodata) - KERNEL_BASE) {
		*(.rodata .rodata.*)
	}
	
	/* Read-write data, page aligned for the .padata section */
	.data ALIGN(0x1000) : AT(ADDR(.data) - KERNEL_BASE) {
		*(.padata)
		*(.data .data.*)
	}
	
	/* Zero-initialised data */
	.bss : AT(ADDR(.bss) - KERNEL_BASE) {
		*(.bss .bss.*)
	}
	
	kernel_end = .;
	
	/DISCARD/ : {
		*(.note .note.*)
	}
}