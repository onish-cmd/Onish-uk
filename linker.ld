ENTRY(_start)

SECTIONS
{
    /* This is the address where QEMU starts reading the kernel */
    . = 0x40000000; 

    /* Put the code (text) here */
    .text : {
        KEEP(*(.text._start)) /* Ensure _start is at the very beginning */
        *(.text .text.*)
    }

    /* Put constants and strings (like "If you see this, You WON!") here */
    .rodata : {
        *(.rodata .rodata.*)
    }

    /* Put global variables here */
    .data : {
        *(.data .data.*)
    }

    /* Put zero-initialized variables here */
    .bss : {
        *(.bss .bss.*)
    }
}
