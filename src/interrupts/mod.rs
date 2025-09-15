use core::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::vga::println;

pub mod keyboard;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub const TIMER_FREQUENCY_HZ: u64 = 1000;
pub const MS_PER_TICK: u64 = 1000 / TIMER_FREQUENCY_HZ;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

static TICK_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.segment_not_present
            .set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault
            .set_handler_fn(stack_segment_fault_handler);

        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(nmi_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded
            .set_handler_fn(bound_range_exceeded_handler);
        idt.device_not_available
            .set_handler_fn(device_not_available_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception
            .set_handler_fn(security_exception_handler);

        idt[InterruptIndex::Timer as u8].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init() {
    configure_timer(TIMER_FREQUENCY_HZ as u16);
    IDT.load();
    unsafe { PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn get_ticks() -> u64 {
    TICK_COUNTER.load(Ordering::Relaxed)
}

pub fn sleep(duration: Duration) {
    let start_ticks = get_ticks();
    let sleep_ticks = ((duration.as_millis() as u64) + MS_PER_TICK - 1) / MS_PER_TICK; // Round up
    let target_ticks = start_ticks + sleep_ticks;

    while get_ticks() < target_ticks {
        core::hint::spin_loop();
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    TICK_COUNTER.fetch_add(1, Ordering::Relaxed);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    keyboard::handle_keyboard_interrupt(scancode);
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    panic!("Page fault");
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT");
    println!("Error Code: {}", error_code);
    println!("{:#?}", stack_frame);
    panic!("General protection fault");
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: INVALID OPCODE");
    println!("{:#?}", stack_frame);
    panic!("Invalid opcode");
}

extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: SEGMENT NOT PRESENT");
    println!("Error Code: {}", error_code);
    println!("{:#?}", stack_frame);
    panic!("Segment not present");
}

extern "x86-interrupt" fn stack_segment_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: STACK SEGMENT FAULT");
    println!("Error Code: {}", error_code);
    println!("{:#?}", stack_frame);
    panic!("Stack segment fault");
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVIDE ERROR");
    println!("{:#?}", stack_frame);
    panic!("Divide error");
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DEBUG");
    println!("{:#?}", stack_frame);
}

extern "x86-interrupt" fn nmi_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: NON-MASKABLE INTERRUPT");
    println!("{:#?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT");
    println!("{:#?}", stack_frame);
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: OVERFLOW");
    println!("{:#?}", stack_frame);
    panic!("Overflow");
}

extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BOUND RANGE EXCEEDED");
    println!("{:#?}", stack_frame);
    panic!("Bound range exceeded");
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DEVICE NOT AVAILABLE");
    println!("{:#?}", stack_frame);
    panic!("Device not available");
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    println!("EXCEPTION: INVALID TSS");
    println!("Error Code: {}", error_code);
    println!("{:#?}", stack_frame);
    panic!("Invalid TSS");
}

extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: VIRTUALIZATION");
    println!("{:#?}", stack_frame);
    panic!("Virtualization exception");
}

extern "x86-interrupt" fn security_exception_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION: SECURITY EXCEPTION");
    println!("Error Code: {}", error_code);
    println!("{:#?}", stack_frame);
    panic!("Security exception");
}

pub fn configure_timer(frequency_hz: u16) {
    use x86_64::instructions::port::Port;

    let divisor = 1193180 / frequency_hz as u32;

    let mut command_port = Port::new(0x43);
    let mut data_port = Port::new(0x40);
    unsafe {
        // configure channel 0, access mode lobyte/hibyte, mode 3
        command_port.write(0x36u8);

        // data port for channel 0
        data_port.write((divisor & 0xFF) as u8); // low byte
        data_port.write(((divisor >> 8) & 0xFF) as u8); // high byte
    }
}
