use crate::Player;
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
        for row in &self.grid {
            for cell in row {
                match cell {
                    Some(Player::X) => print!(" X "),
                    Some(Player::O) => print!(" O "),
                    None => print!(" - "),
                }
            }
            println!()
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

    pub fn has_winner(&self) -> Option<Player> {
        for i in 0..3 {
            if let Some(player) = self.check_line(self.grid[i][0], self.grid[i][1], self.grid[i][2]) {
                return Some(player);
            }
            if let Some(player) = self.check_line(self.grid[0][i], self.grid[1][i], self.grid[2][i]) {
                return Some(player);
            } 
        }
        if let Some(player) = self.check_line(self.grid[0][0], self.grid[1][1], self.grid[2][2]) {
            return Some(player);
        }
        if let Some(player) = self.check_line(self.grid[0][2], self.grid[1][1], self.grid[2][0]) {
            return Some(player);
        }
        None
    }

    fn check_line(&self, cell1: Option<Player>, cell2: Option<Player>, cell3: Option<Player>) -> Option<Player> {
        match (cell1, cell2, cell3) {
            (Some(player1), Some(player2), Some(player3)) if player1 == player2 && player2 == player3 => Some(player1),
            _ => None,
        }
    }

    pub fn is_full(&self) -> bool {
        self.grid.iter().flatten().all(|&cell| cell.is_some())
    }
}