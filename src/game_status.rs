use crate::Player;

/// Represents the current status of the Tic Tac Toe game.
///
/// The game can be in an `Ongoing` state, have a `Winner`, or result in a `Tie`.
pub enum GameStatus {
    Ongoing,
    Winner(Player),
    Tie,
}


