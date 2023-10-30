use std::io::{self, Write};

mod board;
mod player;
mod game_status;

use board::Board;
use player::Player;
use game_status::GameStatus;

/// The main function that drives the Tic Tac Toe game.
///
/// It runs in a loop until the user decides to quit the game. Each loop:
/// 1. Initializes a new game board.
/// 2. Alternates between the two players allowing them to make a move.
/// 3. Checks the game status after every move to determine if there's a winner or a tie.
/// 4. Once a game ends, it prompts the user to play again or quit.
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

/// Clears the terminal screen.
///
/// This function sends specific escape sequences to the terminal to clear it.
/// It's useful to keep the game board view clean between the moves.
fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}