#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Play {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}
impl Player {
    pub fn flip(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Event {
    pub play: Play,
    pub player: Player,
}
impl Event {
    pub fn new(play: Play, player: Player) -> Self {
        Self { play, player }
    }
}
