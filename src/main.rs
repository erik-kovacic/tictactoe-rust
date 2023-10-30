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

    
    loop {
        _board.display_board();
        _board.make_move(_current_player);

        if let Some(winner) = _board.has_winner() {
            println!("{:?} is the winner!", winner);
            break;
        } else if _board.is_full() {
            println!("It's a tie!");
            break;
        }
        
        _current_player.switch(); 
    }
}
