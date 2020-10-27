ENTRY(_init)
OUTPUT_FORMAT(elf64-x86-64)


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

	.init : AT(ADDR(.init)) {
		KEEP( *(.multiboot-header) )
		*(.inittext)
	}

}