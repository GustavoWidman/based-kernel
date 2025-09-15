use alloc::vec::Vec;
use lazy_static::lazy_static;
use pc_keyboard::{HandleControl, KeyCode, Keyboard, ScancodeSet1, layouts};
use spin::{Mutex, RwLock};

use crate::game::event::Play;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore
        ));
    pub static ref EVENT_QUEUE: RwLock<Vec<Play>> = RwLock::new(Vec::new());
}

pub fn handle_keyboard_interrupt(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if key_event.state == pc_keyboard::KeyState::Down {
            match key_event.code {
                KeyCode::Key1 => (*EVENT_QUEUE.write()).push(Play::One),
                KeyCode::Key2 => (*EVENT_QUEUE.write()).push(Play::Two),
                KeyCode::Key3 => (*EVENT_QUEUE.write()).push(Play::Three),
                KeyCode::Key4 => (*EVENT_QUEUE.write()).push(Play::Four),
                KeyCode::Key5 => (*EVENT_QUEUE.write()).push(Play::Five),
                KeyCode::Key6 => (*EVENT_QUEUE.write()).push(Play::Six),
                KeyCode::Key7 => (*EVENT_QUEUE.write()).push(Play::Seven),
                KeyCode::Key8 => (*EVENT_QUEUE.write()).push(Play::Eight),
                KeyCode::Key9 => (*EVENT_QUEUE.write()).push(Play::Nine),
                _ => {}
            }
        }
    }
}
