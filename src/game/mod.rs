use core::time::Duration;

use alloc::{string::ToString, vec::Vec};

pub mod event;
pub mod table;

use super::interrupts::{keyboard::EVENT_QUEUE, sleep};
use super::vga::WRITER;
use event::{Event, Player};
use table::Table;

pub fn run_game() {
    let mut table = Table::new();
    let mut player = Player::X;

    WRITER.lock().draw_table(&table, Vec::new(), None);

    loop {
        if !EVENT_QUEUE.read().is_empty() {
            let mut errors = Vec::new();
            for play in EVENT_QUEUE.write().drain(..) {
                let event = Event::new(play, player);
                match table.play(event) {
                    Ok(_) => player = player.flip(),
                    Err(e) => errors.push(e.to_string()),
                }
            }

            let winner = table.check_wins();
            WRITER.lock().draw_table(&table, errors, winner);

            if winner.is_some() {
                break;
            }
        }

        sleep(Duration::from_millis(1));
    }
}
