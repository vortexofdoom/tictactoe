use itertools::Itertools;
use rand::Rng;

use crate::board::Board;

const THREE_IN_A_ROW: [[usize; 3]; 8] = [[1, 2, 3], [4, 5, 6], [7, 8, 9], [1, 4, 7], [2, 5, 8], [3, 6, 9], [1, 5, 9], [3, 5, 7]];

pub trait IsPlayer {
    fn is_human(&self) -> bool;
}

pub enum PlayerType {
    Human,
    Random,
    FindWinning,
    BlockLosing,
}

impl IsPlayer for PlayerType {
    fn is_human(&self) -> bool {
        match self {
            Self::Human => true,
            _ => false,
        }
    }
}

pub struct Player {
    name: char,
    player: PlayerType,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Player {
    pub fn new(name: char, player: PlayerType) -> Self {
        Self {
            name,
            player,
        }
    }

    pub fn name(&self) -> char {
        self.name
    }

    pub fn get_input(&self, buf: &mut String) -> usize {
        loop {
            buf.clear();
            println!("Player {}, please select an empty cell 1-9: ", self.name);
            std::io::stdin()
                .read_line(buf)
                .expect("could not read line");
            if let Ok(n @ 1..=9) = buf.trim().parse::<usize>() {
                return n;
            }
        }
    }

    pub fn pick_random_open_move(&self, board: &Board) -> usize {
        let open = board
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_numeric())
            .map(|(i, _)| i + 1)
            .collect_vec();

        let guess = rand::thread_rng().gen_range(0..open.len());
        *open.get(guess).unwrap()
    }

    fn check_win_loss(&self, board: &Board) -> Vec<(usize, bool)> {
        let cell = |i| board.get_cell(i).expect("invalid index");
        let check_win_loss = |a,b,c| match [cell(a), cell(b), cell(c)] {
            [Some(x), Some(y), None] => if x == y {
                Some((c, x == self.name))
            } else {
                None
            },
            [Some(x), None, Some(y)] => if x == y {
                Some((b, x == self.name))
            } else {
                None
            },
            [None, Some(x), Some(y)] => if x == y {
                Some((a, x == self.name))
            } else {
                None
            },
            _ => None
        };

        THREE_IN_A_ROW.iter()
            .map(|[a, b, c]| check_win_loss(*a, *b, *c))
            .filter(|i| i.is_some())
            .map(|i| i.unwrap())
            .collect_vec()
    }

    fn only_find_winning(&self, board: &Board) -> usize {
        for (i, b) in self.check_win_loss(board) {
            if b {
                return i;
            }
        }
        self.pick_random_open_move(board)
    }

    fn find_winning_block_losing(&self, board: &Board) -> usize {
        // Prioritize winning over blocking a loss over picking at random
        match self.check_win_loss(board).iter().find_or_first(|(_, b)| *b) {
            Some((best, _)) => *best,
            _ => self.pick_random_open_move(board),
        }
    }

    pub fn make_move(&self, board: &Board, buf: &mut String) -> usize {
        match self.player {
            PlayerType::Human => self.get_input(buf),
            PlayerType::Random => self.pick_random_open_move(board),
            PlayerType::FindWinning => self.only_find_winning(board),
            PlayerType::BlockLosing => self.find_winning_block_losing(board),
        }
    }
}
