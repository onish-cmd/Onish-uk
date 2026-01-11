#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use core::arch::asm;

// --- HARDWARE ADDRESSES ---
#[cfg(feature = "qemu")]
const UART0_BASE: *mut u8 = 0x09000000 as *mut u8; 

#[cfg(not(feature = "qemu"))]
const UART0_BASE: *mut u8 = 0xff130000 as *mut u8;

pub fn uart_punc(c: u8) {
    let ptr = UART0_BASE;
    unsafe { core::ptr::write_volatile(ptr, c)}
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
        "mov sp, #0x48000000",
        "bl kmain",
        options(noreturn)
        );
    }
}

#[no_mangle]
pub fn kmain() -> ! {
    unsafe {
        let uart_cr = (0x09000000 + 0x30) as *mut u32;
        let fr = (0x0900_0000 + 0x18) as *const u32; // Flag Register
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