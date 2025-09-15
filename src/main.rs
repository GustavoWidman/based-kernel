#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use talc::{ClaimOnOom, Span, Talc, Talck};

use crate::{
    interrupts::sleep,
    vga::{WRITER, println},
};
use core::{panic::PanicInfo, time::Duration};
mod game;
mod interrupts;
mod vga;

// 64kb heap arena
static mut ARENA: [u8; 65536] = [0; 65536];

#[global_allocator]
static ALLOCATOR: Talck<spin::Mutex<()>, ClaimOnOom> =
    Talc::new(unsafe { ClaimOnOom::new(Span::from_array(core::ptr::addr_of!(ARENA).cast_mut())) })
        .lock();

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    interrupts::init();
    println!("Hello from Rust kernel!");
    sleep(Duration::from_millis(100));
    println!("Kernel booted successfully!");
    sleep(Duration::from_millis(100));
    println!("Booting game...");
    sleep(Duration::from_millis(500));

    game::run_game();

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut writer = WRITER.lock();
    writer.set_color(0x0c);
    println!("KERNEL PANIC! {}", _info);
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
