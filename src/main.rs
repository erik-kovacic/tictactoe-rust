mod board;
mod player;
mod game_status;

use board::Board;
use player::Player;
use game_status::GameStatus;

fn main() {
    let mut _board = Board::new();
    let mut _current_player = Player::X;
    let mut _game_status = GameStatus::Ongoing;

    _board.display_board()
}
