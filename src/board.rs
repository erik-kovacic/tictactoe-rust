pub struct Board {
    grid: [[Option<Player>; 3]; 3]
}

impl Board {
    pub fn newBoard() -> Self {
        Board {
            grid: [[None; 3]; 3]
        }
    }

    
}