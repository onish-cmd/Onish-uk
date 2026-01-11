#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

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
pub extern "C" fn _start() -> ! {
    print("\nIf you see this, You WON!\n");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("KERNEL PANIC!");
    loop {}
}