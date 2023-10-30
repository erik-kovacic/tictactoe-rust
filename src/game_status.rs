#![allow(dead_code)]
use crate::Player;

pub enum GameStatus {
    Ongoing,
    Winner(Player),
    Tie,
}


