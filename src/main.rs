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

    
    while let GameStatus::Ongoing = _game_status {
        _board.display_board();
        _board.make_move(_current_player);
        
        _game_status = _board.check_game_status();
    
        if let GameStatus::Ongoing = _game_status {
            _current_player.switch();
        }
    }

    match _game_status {
        GameStatus::Winner(player) => println!("{:?} is the winner!", player),
        GameStatus::Tie => println!("It's a tie!"),
        _ => {}
    }
}
