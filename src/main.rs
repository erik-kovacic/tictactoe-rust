use std::io::{self, Write};

mod board;
mod player;
mod game_status;

use board::Board;
use player::Player;
use game_status::GameStatus;

fn main() {
    loop {
        let mut board = Board::new();
        let mut current_player = Player::X;
        let mut game_status = GameStatus::Ongoing;

        while let GameStatus::Ongoing = game_status {
            clear_screen();
            board.display_board();
            board.make_move(current_player);
        
            game_status = board.check_game_status();
    
            if let GameStatus::Ongoing = game_status {
                current_player.switch();
            }
        }

        match game_status {
            GameStatus::Winner(player) => println!("{:?} is the winner!", player),
            GameStatus::Tie => println!("It's a tie!"),
            _ => {}
        }

        // Ask to play again
        println!("Would you like to play again? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            break;
        }
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}