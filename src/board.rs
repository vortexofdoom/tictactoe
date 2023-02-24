use grid::Grid;
use anyhow::{anyhow, Result};

const EMPTY: [char; 9]  = [
    '1', '2', '3', 
    '4', '5', '6', 
    '7', '8', '9'];

#[derive(Debug, Clone)]
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

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.grid == other.grid
    }
}

impl Board {
    pub const P1: char = 'X';
    pub const P2: char = 'O';

    pub fn player(first: bool) -> char {
        if first { Board::P1 } else { Board::P2 }
    }

    pub fn new() -> Self {
        Self {         
            grid: Grid::from_vec(Vec::from(EMPTY), 3)
        }
    }

    pub fn from_vec(vec: Vec<char>) -> Self {
        Self {
            grid: Grid::from_vec(vec, 3),
        }
    }

    pub fn get_open_spaces(&self) -> Vec<usize> {
        self.iter()
            .enumerate()
            .filter(|(_, c)| c.is_numeric())
            .map(|(i, _)| i + 1)
            .collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.grid.iter()
    }

    pub fn reset(&mut self) {
        for (i, c) in self.grid.iter_mut().enumerate() {
            *c = EMPTY[i];
        }
    }

    fn state(&self) -> Board {
        Self::from_vec(self
            .iter()
            .map(|c| match c {
                'X' => '1',
                'O' => '2',
                _ => '0',
            })
            .collect()
        )
    }

    pub fn flip_v(&self) -> Board {
        let row0 = self.grid.iter_row(0);
        let row1 = self.grid.iter_row(1);
        let row2 = self.grid.iter_row(2);
        Self::from_vec(row2
            .chain(row1)
            .chain(row0)
            .cloned()
            .collect()
        )
        
    }

    pub fn flip_h(&self) -> Board {
        let row0 = self.grid.iter_row(0).rev();
        let row1 = self.grid.iter_row(1).rev();
        let row2 = self.grid.iter_row(2).rev();
        Self::from_vec(row0
            .chain(row1)
            .chain(row2)
            .cloned()
            .collect()
        )
    }

    pub fn transpose(&self) -> Board {
        Board { grid: self.grid.transpose() }
    } 

    pub fn state_key(&self) -> String {
        let base = self.state();
        let mut states: Vec<String> = vec![];
        states.push(base.clone().iter().collect());
        // Flip vertically
        states.push(base.flip_v().iter().collect());
        // Flip horizontally
        states.push(base.flip_h().iter().collect());
        // Rotate 90 degrees L
        states.push(base.transpose().flip_v().iter().collect());
        // Rotate 180 degrees
        states.push(base.flip_h().flip_v().iter().collect());
        // Rotate 90 degrees R
        states.push(base.transpose().flip_h().iter().collect());
        states.push(base.transpose().iter().collect());
        states.push(base.transpose().flip_v().flip_h().iter().collect());
        states.into_iter().min().unwrap()
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

    pub fn set_cell(&mut self, i: usize, first: bool) -> Result<()> {
        let player = if first { Self::P1 } else { Self::P2 };
        if self.get_cell(i)?.is_none() {
            let i = i - 1;
            Ok(self.grid[i / 3][i % 3] = player)
        } else {
            Err(anyhow!("cell already occupied"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_have_used_nalgebra() {
        let board = Board::new();
        assert_eq!(
            board.flip_h(), 
            Board::from_vec(vec![
                '3', '2', '1',
                '6', '5', '4',
                '9', '8', '7'
            ])
        );
        
        assert_eq!(
            board.flip_v(), 
            Board::from_vec(vec![
                '7', '8', '9',
                '4', '5', '6', 
                '1', '2', '3'
            ])
        );

        assert_eq!(
            board.transpose().flip_v(), 
            Board::from_vec(vec![
                '3', '6', '9',
                '2', '5', '8', 
                '1', '4', '7'
            ])
        );

        assert_eq!(
            board.flip_h().flip_v(), 
            Board::from_vec(vec![
                '9', '8', '7',
                '6', '5', '4', 
                '3', '2', '1'
            ])
        );

        assert_eq!(
            board.transpose().flip_h(), 
            Board::from_vec(vec![
                '7', '4', '1',
                '8', '5', '2', 
                '9', '6', '3'
            ])
        );

        assert_eq!(
            board.transpose().flip_v().flip_h(),
            Board::from_vec(vec![
                '9', '6', '3',
                '8', '5', '2', 
                '7', '4', '1'
            ])
        );
    }
}