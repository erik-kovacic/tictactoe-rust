#[derive(Copy, Clone)]
#[derive(Debug, PartialEq)]

pub enum Player {
    X, O
}

impl Player {
    pub fn switch(&mut self) {
        match *self {
            Player::X => *self = Player::O,
            Player::O => *self = Player::X,
        }
    }
}