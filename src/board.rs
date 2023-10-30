use crate::Player;
use crate::GameStatus;
use std::io;

/// Represents the Tic Tac Toe game board.
///
/// The game board consists of a 3x3 grid, which can contain `Player::X`, `Player::O`, or be empty (None).
pub struct Board {
    grid: [[Option<Player>; 3]; 3]
}

impl Board {
    /// Creates a new, empty game board.
    pub fn new() -> Self {
        Board {
            grid: [[None; 3]; 3]
        }
    }

    /// Displays the current state of the game board with borders.
    ///
    /// Players are represented as 'X' and 'O'. Empty cells are displayed with spaces.
    pub fn display_board(&self) {
        println!("+---+---+---+");
        for row in &self.grid {
            print!("|");
            for cell in row {
                match cell {
                    Some(Player::X) => print!(" X "),
                    Some(Player::O) => print!(" O "),
                    None => print!("   "),
                }
                print!("|");
            }
            println!();
            println!("+---+---+---+");
        }
    }

    /// Checks if the cell at the given row and column is empty.
    fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        self.grid[row][col].is_none()
    }

    /// Prompts the user for input and returns a valid usize (0, 1, or 2).
    ///
    /// Continues to prompt the user until a valid input is given.
    fn get_input() -> usize {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<usize>() {
                Ok(num) if num < 3 => return num,
                _ => println!("Invalid input. Please enter 0, 1, or 2."),
            }
        }
    }

    /// Allows the specified player to make a move on the board.
    ///
    /// Continues to prompt the player for a valid move until a valid and unoccupied cell is chosen.
    /// Returns true once a valid move is made.    
    pub fn make_move(&mut self, player: Player) -> bool {
        loop {
            println!("Player {:?}, enter row (0, 1, 2): ", player);
            let row = Board::get_input();
            println!("Player {:?}, enter column (0, 1, 2): ", player);
            let col = Board::get_input();

            if self.is_cell_empty(row, col) {
                self.grid[row][col] = Some(player);
                return true;
            } else {
                println!("Cell ({}, {}) is already occupied. Try again.", row, col);
            }
        }
    }

    /// Checks the current status of the game.
    ///
    /// Determines if the game has a winner, is a tie, or is still ongoing.
    pub fn check_game_status(&self) -> GameStatus {
        if let Some(winner) = self.has_winner() {
            GameStatus::Winner(winner)
        } else if self.is_full() {
            GameStatus::Tie
        } else {
            GameStatus::Ongoing
        }
    }

    /// Determines if there's a winner on the current board.
    ///
    /// Checks rows, columns, and diagonals to see if any player has three in a row.
    /// Returns `Some(Player)` if there's a winner, and `None` otherwise.
    fn has_winner(&self) -> Option<Player> {
        for row in &self.grid {
            if let Some(&first) = row.first() {
                if row.iter().all(|&cell| cell == first) {
                    return first;
                }
            }
        }

        for col in 0..3 {
            let first = self.grid[0][col];
            if self.grid.iter().all(|&row| row[col] == first) {
                return first;
            }
        }

        let main_diag = self.grid[0][0];
        if (1..3).all(|i| self.grid[i][i] == main_diag) {
            return main_diag;
        }

        let anti_diag = self.grid[0][2];
        if (1..3).all(|i| self.grid[i][2 - i] == anti_diag) {
            return anti_diag;
        }

        None
    }

    /// Checks if the board is full, indicating a tie if there's no winner.
    ///
    /// Returns `true` if all cells are occupied, `false` otherwise.
    pub fn is_full(&self) -> bool {
        self.grid.iter().flatten().all(|&cell| cell.is_some())
    }
}