ENTRY(_start)

SECTIONS
{
	/* . = 0x8000; */
	. = 0x2000000;

	__start = .;
	.text ALIGN(4K) :
	{
		/* put the .text.boot section first */
		KEEP(*(.text.boot))
		*(.text)
	}

	.rodata ALIGN(4K) :
	{
		*(.rodata)
	}

	.data ALIGN(4K) :
	{
		*(.data)
	}
	. = ALIGN(4K);
	__data_end = .;

	.bss ALIGN(4K) :
	{
		__bss_start = .;
		*(.bss)
	}
	. = ALIGN(4K);
	__bss_end = .;
	__end = .;
}
