#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]

// Imports
use core::panic::PanicInfo;
use core::arch::asm;
use core::arch::global_asm;
extern crate fdt;

global_asm!(
    r#"
    .section .text._start
    .global _start
    _start:
        mrs r0, cpsr
        bic r0, r0, #0x1F
        orr r0, r0, #0x13
        msr cpsr_c, r0

        ldr r0, =_start
        adr r1, _start
        sub r12, r1, r0

        ldr r3, =__stack_top
        add sp, r3, r12

        ldr r1, =__bss_start
        add r1, r1, r12
        ldr r2, =__bss_end
        add r2, r2, r12
        mov r3, #0
    clear_bss:
        cmp r1, r2
        strlo r3, [r1], #4
        blo clear_bss

        mov r0, r2
        mov r1, r12
        bl kmain

    halt:
        wfi
        b halt
    "#
);

static mut UART_BASE: *mut u8 = 0x09000000 as *mut u8;

pub fn uart_punc(c: u8) {
    unsafe {
        core::ptr::write_volatile(UART_BASE, c)
    }
}

fn print(s: &str) {
    for b in s.bytes() {
        uart_punc(b)
    }
}

#[no_mangle]
pub fn kmain(dtb_ptr: usize, delta: usize) -> ! {
    unsafe {
    UART_BASE = (UART_BASE as usize + delta) as *mut u8;
    if let Ok(fdt) = fdt::Fdt::from_ptr(dtb_ptr as *const u8) {
        let uart_node = fdt.find_compatible(&["arm,pl011"])
        .or_else(|| fdt.find_compatible(&["snps,dw-apb-uart"]));
        if let Some(node) = uart_node {
            if let Some(reg) = node.reg().and_then(|mut r| r.next()) {
                UART_BASE = reg.starting_address as *mut u8;
            }
    }
    }
}
    unsafe {
        let uart_cr = UART_BASE.add(0x30) as *mut u32;
        let fr = UART_BASE.add(0x18) as *const u32; // Flag Register
        while (core::ptr::read_volatile(fr) & 0x20) != 0 {
            core::hint::spin_loop()
        }
        core::ptr::write_volatile(uart_cr, 0x301);
    }

    print("\n-- Onish-Kernel --\n");

    loop {
        unsafe {
            core::arch::asm!("wfi")
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("KERNEL PANIC!");
    loop {}
}