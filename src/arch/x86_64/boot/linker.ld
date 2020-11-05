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




	.init ALIGN(0x1000): AT(ADDR(.init)) {
		KEEP( *(.multiboot-header) )
		*(.inittext)

		/* Here are some things required for bootstrapping */

	    . = ALIGN(0x1000);
		page_table_start = .; 
		p4 = .;
		. += 0x1000;
		p3 = .;
		. += 0x1000; 
		page_table_end = .;
		// Very little stack, be carefull
		boot_stack_bottom = .;
		. += 64;
		boot_stack_top = .;		
		boot_info_addr = .;
		. += 4;
		

	}
	
	/* We define the load offset with this in mind :
	   We want the LMA to be under 4GiB while the VMA should be
	   in higher half.
	 */

	init_end = ALIGN(0x1000);
	LOAD_OFFSET = KERNEL_BASE;

	. += KERNEL_BASE;
	phys_kern_start = . - LOAD_OFFSET;

	.text ALIGN(0x1000) : AT(ADDR(.text) - LOAD_OFFSET) {
		*(.text .text.*)
		_etext = .;
	}

	/* read-only data, page aligned to allow use of the no-execute feature */
	.rodata ALIGN(0x1000) : AT(ADDR(.rodata) - LOAD_OFFSET) {
		*(.rodata .rodata.*)
		. = ALIGN(0x1000);
		_erodata = .;
	}

	/* The follow snippet forces the LMA to be equal to the VMA.. Why ?
	 `.data ALIGN(0x1000): AT(ADDR(.data) - LOAD_OFFSET)`
	  A rapid fix is to page align the end of .rodata */
	 
	.data : AT(ADDR(.data) - LOAD_OFFSET ){
		*(.padata)
		*(.data .data.*)
		._edata = .;
	}
	
	/* Zero-initialised data */
	.bss : AT(ADDR(.bss) - KERNEL_BASE) {
		*(.bss .bss.*)
	}

	phys_kern_end = . - LOAD_OFFSET;
	kernel_end = .;
	
	/DISCARD/ : {
		*(.note .note.*)
	}
}