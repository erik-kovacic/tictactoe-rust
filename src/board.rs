use crate::Player;
use crate::GameStatus;
use std::io;

pub struct Board {
    grid: [[Option<Player>; 3]; 3]
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[None; 3]; 3]
        }
    }

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

    fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        self.grid[row][col].is_none()
    }

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

    pub fn check_game_status(&self) -> GameStatus {
        if let Some(winner) = self.has_winner() {
            GameStatus::Winner(winner)
        } else if self.is_full() {
            GameStatus::Tie
        } else {
            GameStatus::Ongoing
        }
    }

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

    pub fn is_full(&self) -> bool {
        self.grid.iter().flatten().all(|&cell| cell.is_some())
    }
}