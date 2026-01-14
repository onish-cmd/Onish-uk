ENTRY(_start)

SECTIONS
{
    /* We link at 0. The Assembly/PIC handles the real address! */
    . = 0x00000000;

    .text : {
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    
    /* VERY IMPORTANT: PIC uses the Global Offset Table */
    .got : { *(.got*) }

    .data : { *(.data .data.*) }

    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    . = ALIGN(8);
    .stack : {
        . += 0x10000; /* 64KB stack */
        __stack_top = .;
    }
}