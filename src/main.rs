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
pub extern "C" fn _start() -> ! {
    unsafe {
        // Move stack pointer to #0x48000000
        asm!("mov sp, #0x48000000");

        // Enable UART
        let uart_cr = (0x09000000 + 0x30) as *mut u32;
        core::ptr::write_volatile(uart_cr, 0x301); // 0x301 sets UARTEN (bit 0), TXE (bit 8), and RXE (bit 9)
    }
    print("\n-- Onish-µK 0.0.1 --\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("KERNEL PANIC!");
    loop {}
}