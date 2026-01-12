#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]

// Imports
use core::panic::PanicInfo;
use core::arch::asm;
extern crate fdt;

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
#[link_section = ".text._start"]
pub extern "C" fn _start() {
    unsafe {
        // Move stack pointer to #0x48000000
        asm!(
        "mov r0, r2",
        "mov sp, #0x48000000",
        "bl kmain",
        options(noreturn)
        );
    }
}

#[no_mangle]
pub fn kmain(dtb_ptr: usize) -> ! {
    unsafe {
    if let Ok(fdt) = fdt::Fdt::from_ptr(dtb_ptr as *const u8) {
        let uart_node = fdt.find_compatible(&["arm,pl011"])
        .or_else(|| fdt.find_compatible(&["snp,dw-apb-uart"]));
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