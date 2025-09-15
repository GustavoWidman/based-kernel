use core::usize;

use anyhow::Result;

use crate::game::event::{Event, Player};

pub struct Table {
    pub state: [Option<Player>; 9],
}

#[derive(Clone, Copy)]
pub struct Win(pub usize, pub usize, pub usize);

impl Table {
    pub fn new() -> Self {
        Self { state: [None; 9] }
    }

    pub fn play(&mut self, event: Event) -> Result<()> {
        let index = event.play as usize;
        if index >= 9 {
            return Err(anyhow::anyhow!("Index out of bounds"));
        }
        if self.state[index].is_some() {
            return Err(anyhow::anyhow!("Cell already occupied"));
        }
        self.state[index] = Some(event.player);
        Ok(())
    }

    pub fn check_wins(&self) -> Option<(Player, Win)> {
        let lines = [
            // rows
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            // columns
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            // diagonals
            (0, 4, 8),
            (2, 4, 6),
        ];

        for &(a, b, c) in &lines {
            if let (Some(player_a), Some(player_b), Some(player_c)) =
                (self.state[a], self.state[b], self.state[c])
            {
                if player_a == player_b && player_b == player_c {
                    return Some((player_a, Win(a, b, c)));
                }
            }
        }

        None
    }
}
