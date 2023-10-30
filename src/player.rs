#![allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Player {
    X, O
}

impl Player {
    pub fn switch(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}