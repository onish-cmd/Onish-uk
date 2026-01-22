#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(rustc_private)] // You can also ignore this error since the github action uses nightly

use core::panic::PanicInfo;
use core::arch::global_asm;
extern crate fdt;
extern crate compiler_builtins; // You can safely ignore this unresolved extern crate

global_asm!(
    r#"
    .section .text._start
    .global _start
    _start:
        @ 1. Switch to Supervisor Mode (EL1)
        mrs r0, cpsr
        bic r0, r0, #0x1F
        orr r0, r0, #0x13
        msr cpsr_c, r0

        @ 2. Calculate the Delta
        @ r2 currently holds the FDT pointer from U-Boot - DO NOT LOSE IT
        adr r0, _start
        ldr r1, =_start
        sub r12, r0, r1      @ r12 = Real Address - Linked Address

        @ 3. Relocate the GOT (Global Offset Table)
        @ This is what fixes 'static mut UART_BASE'
        ldr r4, =__got_start
        ldr r5, =__got_end
        add r4, r4, r12      @ Adjust GOT start to real RAM
        add r5, r5, r12      @ Adjust GOT end to real RAM

    relocate_got:
        cmp r4, r5
        bge relocation_done
        ldr r6, [r4]         @ Load the absolute pointer stored in GOT
        add r6, r6, r12      @ Offset it by our Delta
        str r6, [r4], #4     @ Store it back and move to next entry
        b relocate_got

    relocation_done:
        @ 4. Set up Stack
        ldr r3, =__stack_top
        add sp, r3, r12      @ Offset the stack pointer

        @ 5. Clear BSS
        ldr r1, =__bss_start
        add r1, r1, r12
        ldr r2, =__bss_end
        add r2, r2, r12
        mov r3, #0
    clear_bss:
        cmp r1, r2
        strlo r3, [r1], #4
        blo clear_bss

        @ 6. Jump to Rust
        @ U-Boot put FDT in r2. Let's move it to r0 for kmain(dtb_ptr, delta)
        mov r0, r2           @ Arg 0: FDT Pointer
        mov r1, r12          @ Arg 1: Delta
        bl kmain

    halt:
        .inst 0xe320f003
        b halt
    "#
);

#[no_mangle]
static mut UART_BASE: *mut u8 = 0x09000000 as *mut u8;


pub fn uart_punc(c: u8) {
    unsafe {
        // The compiler now generates PC-relative code to find this address!
        let base = UART_BASE; 
        
        // Wait for TX FIFO (Flag Register offset 0x18, bit 5)
        while (core::ptr::read_volatile(base.add(0x18) as *const u32) & 0x20) != 0 {
            core::hint::spin_loop();
        }
        core::ptr::write_volatile(base, c);
    }
}

fn print(s: &str) {
    for b in s.bytes() {
        uart_punc(b);
    }
}

#[no_mangle]
pub fn kmain(dtb_ptr: usize, _delta: usize) -> ! {
    // 2. Discover real UART via FDT
    unsafe {
        if let Ok(fdt) = fdt::Fdt::from_ptr(dtb_ptr as *const u8) {
            let uart_node = fdt.find_compatible(&["arm,pl011"])
                .or_else(|| fdt.find_compatible(&["snps,dw-apb-uart"]));
            
            if let Some(node) = uart_node {
                if let Some(reg) = node.reg().and_then(|mut r| r.next()) {
                    // Just update the static mut! PIC handles the rest.
                    UART_BASE = reg.starting_address as *mut u8;
                }
            }
        }
    }

        unsafe {
            let base = UART_BASE;
            core::ptr::write_volatile(base.add(0x30) as *mut u32, 0x301);
        }
    print("-- Onish-Kernel: PIC Mode Active --\n\r");

    loop {
        unsafe { core::arch::asm!(
        ".inst 0xe320f003",
        options(nomem, nostack)
    ) }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("KERNEL PANIC!");
    loop {
        unsafe { core::arch::asm!(
            ".inst 0xe320f003",
            options(nomem, nostack)
        ) }
    }
}