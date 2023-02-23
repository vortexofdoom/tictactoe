use grid::Grid;
use anyhow::{anyhow, Result};

pub struct Board {
    grid: Grid<char>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = &self.grid;
        let print_row = |row| format!("{}|{}|{}\n", grid[row][0], grid[row][1], grid[row][2]);
        write!(f, "{}-+-+-\n{}-+-+-\n{}\n", print_row(0), print_row(1), print_row(2))
    }
}

impl Board {
    pub fn new() -> Self {
        Self { grid: Grid::from_vec(vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'], 3) }
    }

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.grid.iter()
    }

    pub fn reset(&mut self) {
        let mut cell = 1;
        for i in self.grid.iter_mut() {
            *i = char::from(cell);
            cell += 1;
        }
    }

    pub fn check_matches(&self) -> Option<char> {
        // no one likes typing self
        let board = &self.grid;
        // first check diagonals
        let diag = |a: char, b: char| a == b && a == board[1][1];
        if diag(board[0][0], board[2][2]) { 
            return Some(board[0][0]);
        }
        if diag(board[0][2], board[2][0]) {
            return Some(board[0][2]);
        }

        // Then check rows and columns
        for i in 0..3 {
            // (i, i) is the coordinate that will be common to both row and column i for comparison
            let cell = board[i][i];
            if board.iter_row(i).all(|c| c == &cell) || board.iter_col(i).all(|c| c == &cell) {
                return Some(cell);
            }
        }

        // No match found
        None
    }
    
    pub fn is_full(&self) -> bool {
        self.grid
            .iter()
            .all(|c| c.is_numeric())
    }

    pub fn get_cell(&self, i: usize) -> Result<Option<char>> {
        if i == 0 || i > 9 {
            return Err(anyhow!("Number selected must be between 1 and 9"));
        }
        let row = (i - 1) / 3;
        let col = (i - 1) % 3;
        if let Some(&c) = self.grid.get(row, col) {
            if c.is_alphabetic() {
                Ok(Some(c))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn set_cell(&mut self, i: usize, player: char) -> Result<()> {
        if self.get_cell(i)?.is_none() {
            let i = i - 1;
            Ok(self.grid[i / 3][i % 3] = player)
        } else {
            Err(anyhow!("cell already occupied"))
        }
    }
}