#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

// VGA buffer constants
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    clear_screen();

    print_string(b"Hello from Rust kernel!", 0, 0, 0x07);
    print_string(b"Kernel booted successfully!", 1, 0, 0x0F);

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

fn clear_screen() {
    unsafe {
        for i in 0..(VGA_WIDTH * VGA_HEIGHT * 2) {
            *VGA_BUFFER.add(i) = if i % 2 == 0 { b' ' } else { 0x07 };
        }
    }
}

fn print_string(text: &[u8], row: usize, col: usize, color: u8) {
    unsafe {
        let offset = (row * VGA_WIDTH + col) * 2;
        for (i, &byte) in text.iter().enumerate() {
            let pos = offset + i * 2;
            if pos < VGA_WIDTH * VGA_HEIGHT * 2 {
                *VGA_BUFFER.add(pos) = byte;
                *VGA_BUFFER.add(pos + 1) = color;
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print_string(b"KERNEL PANIC!", 2, 0, 0x4F);
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {
    loop {}
}
