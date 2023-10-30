#[derive(Copy, Clone)]
#[derive(Debug, PartialEq)]

/// Represents a player in the Tic Tac Toe game.
///
/// A player can be either `X` or `O`.
pub enum Player {
    X, O
}

impl Player {
    /// Switches the current player to the other player.
    ///
    /// If the current player is `X`, they will become `O` and vice versa.
    pub fn switch(&mut self) {
        match *self {
            Player::X => *self = Player::O,
            Player::O => *self = Player::X,
        }
    }
}