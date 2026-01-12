ENTRY(_start)

SECTIONS
{
    /* We link at 0. The Assembly calculates the real address! */
    . = 0x00000000;

    .text : {
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    .rodata : { *(.rodata .rodata.*) }
    .data   : { *(.data .data.*) }

    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        __bss_end = .;
    }

    . = ALIGN(8);
    .stack : {
        . += 0x10000; /* 64KB stack */
        __stack_top = .;
    }
}