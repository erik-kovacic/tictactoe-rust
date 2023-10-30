use crate::Player;

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
}