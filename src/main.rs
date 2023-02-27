use std::collections::HashMap;

use anyhow::Result;
use board::Board;
use itertools::Itertools;
use rand::Rng;

mod board;

const THREE_IN_A_ROW: [[usize; 3]; 8] = [[1, 2, 3], [4, 5, 6], [7, 8, 9], [1, 4, 7], [2, 5, 8], [3, 6, 9], [1, 5, 9], [3, 5, 7]];

#[derive(Debug, Clone, Copy)]
enum Player {
    Human,
    Random,
    FindWinning,
    BlockLosing,
    Optimal,
}

struct Game {
    board: Board,
    p1_turn: bool,
    done: bool,
    p1: Player,
    p2: Player,
    cache: HashMap<String, i32>,
}

impl Game {
    pub fn get_y_n_input(msg: &str, buf: &mut String) -> bool {
        loop {
            buf.clear();
            println!("{msg} Y/N: ");
            std::io::stdin()
                .read_line(buf)
                .expect("failed to read line");
    
            match buf.trim().to_lowercase().as_str() {
                "y" | "yes" => return true,
                "n" | "no" => return false,
                _ => {
                    println!("Invalid input! ");
                    continue;
                }
            }
        }
    }

    pub fn new() -> Self {
        Game { 
            board: Board::new(),
            p1: Player::Human,
            p2: Player::Human,
            p1_turn: true,
            done: false,
            cache: HashMap::new(),
        }
    }

    pub fn set_player(&mut self, buf: &mut String) -> Result<()> {
        let first = self.p1_turn;
        let player = if first {
            &mut self.p1
        } else {
            &mut self.p2
        };
        let query = format!("Player {} human?", Board::player(first));

        if !Self::get_y_n_input(&query, buf) {
            loop {
                buf.clear();
                println!("Select AI difficulty (1-3): ");
                std::io::stdin()
                    .read_line(buf)
                    .expect("could not read line");
                match buf.trim().parse::<i32>() {
                    Ok(1) => *player = Player::Random,
                    Ok(2) => *player = Player::FindWinning,
                    Ok(3) => *player = Player::BlockLosing,
                    Ok(4) => *player = Player::Optimal,
                    _ => continue,
                }
                break;
            }
        }
        self.p1_turn = !self.p1_turn;
        Ok(())
    }

    pub fn get_move(&mut self, player: Player, buf: &mut String) -> usize {
        match player {
            Player::Human => self.get_input(buf),
            Player::Random => self.pick_random_open_move(),
            Player::FindWinning => self.only_find_winning(),
            Player::BlockLosing => self.find_winning_block_losing(),
            Player::Optimal => self.pick_optimal_move(),
        }
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.p1_turn = true;
        self.done = false;
    }

    fn curr_player(&self) -> Player {
        if self.p1_turn {
            self.p1
        } else {
            self.p2
        }
    }

    pub fn render(&self) {
        print!("\x1b[H\x1b[J");
        println!("{}", self.board);
    }

    pub fn run(&mut self, buf: &mut String) {
        while !self.done {
            self.render();
            let input = self.get_move(self.curr_player(), buf);
            self.board.set_cell(input, self.p1_turn).unwrap();
            if let Some(winner) = self.board.check_matches() {
                self.done = true;
                self.render();
                println!("Player {winner} wins!");
            } else if self.board.is_full() {
                self.done = true;
                self.render();
                println!("draw!");
            }
            self.p1_turn = !self.p1_turn;
        }
    }

    // move input functions
    pub fn get_input(&mut self, buf: &mut String) -> usize {
        loop {
            buf.clear();
            println!("Player {}, please select an empty cell 1-9: ", Board::player(self.p1_turn));
            std::io::stdin()
                .read_line(buf)
                .expect("could not read line");
            if let Ok(n @ 1..=9) = buf.trim().parse::<usize>() {
                if self.board.get_open_spaces().contains(&n){
                    return n;
                }
            }
        }
    }

    pub fn pick_random_open_move(&self) -> usize {
        let open = self.board.get_open_spaces();
        let guess = rand::thread_rng().gen_range(0..open.len());
        *open.get(guess).unwrap()
    }

    fn check_win_loss(&self) -> Vec<(usize, bool)> {
        let cell = |i| self.board.get_cell(i).expect("invalid index");
        let check_win_loss = |a,b,c| match [cell(a), cell(b), cell(c)] {
            [Some(x), Some(y), None] => if x == y {
                Some((c, x == Board::player(self.p1_turn)))
            } else {
                None
            },
            [Some(x), None, Some(y)] => if x == y {
                Some((b, x == Board::player(self.p1_turn)))
            } else {
                None
            },
            [None, Some(x), Some(y)] => if x == y {
                Some((a, x == Board::player(self.p1_turn)))
            } else {
                None
            },
            _ => None
        };

        THREE_IN_A_ROW.iter()
            .map(|[a, b, c]| check_win_loss(*a, *b, *c))
            .filter(|i| i.is_some())
            .map(|i| i.unwrap())
            .collect()
    }
    
    fn only_find_winning(&self) -> usize {
        for (i, b) in self.check_win_loss() {
            if b {
                return i;
            }
        }
        self.pick_random_open_move()
    }

    fn find_winning_block_losing(&self) -> usize {
        // Prioritize winning over blocking a loss over picking at random
        match self.check_win_loss().iter().find_or_first(|(_, b)| *b) {
            Some((best, _)) => *best,
            None => self.pick_random_open_move(),
        }
    }

    fn pick_optimal_move(&mut self) -> usize {
        let first = self.p1_turn;
        let open = self.board.get_open_spaces();
        let scores = open
            .iter()
            .map(|i| {
                let mut board = self.board.clone();
                board.set_cell(*i, first).unwrap();
                (i, self.minimax_score(!first, &board))
            });
        if first {
            scores.max_by_key(|(_, m)| *m)
            .map(|(i, _)| *i)
            .unwrap()
        } else {
            scores.min_by_key(|(_, m)| *m)
            .map(|(i, _)| *i)
            .unwrap()
        }
    }

    fn minimax_score(&mut self, first: bool, board: &Board) -> i32 {
        let state = board.state_key();
        if let Some(&s) = self.cache.get(&state) {
            return s;
        }
        if let Some(w) = board.check_matches() {
            match w {
                'X' => return 10,
                _ => return -10,
            }
        } else if board.is_full() {
            return 0;
        };
        let options = board.get_open_spaces();
        let res = options
            .iter()
            .map(|i| {
                let mut board = board.clone();
                board.set_cell(*i, first).unwrap();
                self.minimax_score(!first, &board)
            });
        let score = match first {
            true => res.max().unwrap(),
            false => res.min().unwrap(),
        };
        self.cache.insert(state, score);
        score
    }
}



fn main() {
    let mut buf = String::new();
    let mut game = Game::new();
    println!("Welcome to Tic Tac Toe!\n");
    println!("Press Enter to start");
    std::io::stdin()
        .read_line(&mut buf)
        .expect("could not read line");
    loop {
        game.set_player(&mut buf).unwrap();
        game.set_player(&mut buf).unwrap();
        game.run(&mut buf);
        if !Game::get_y_n_input("Play again?", &mut buf) {
            break;
        }
        game.reset();
    }
}