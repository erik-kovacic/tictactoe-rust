use crate::Player;

pub enum GameStatus {
    Ongoing,
    Winner(Player),
    Tie,
}


