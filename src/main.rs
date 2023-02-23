use anyhow::{anyhow, Result};
use board::Board;
use input::Player;

mod board;
mod input;

struct Game {
    board: Board,
    p1_turn: bool,
    done: bool,
    p1: Player,
    p2: Player,
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
        use input::PlayerType::*;
        Game { 
            board: Board::new(),
            p1: Player::new('X', Human),
            p2: Player::new('O', Human),
            p1_turn: true,
            done: false,
        }
    }

    pub fn set_player(&mut self, player: u32, buf: &mut String) -> Result<()> {
        if player == 0 || player > 2 {
            return Err(anyhow!("invalid player id"));
        }
        let (player, name) = if player == 1 {
            (&mut self.p1, 'X')
        } else {
            (&mut self.p2, 'O')
        };

        let query = format!("Player {name} human?");

        if !Self::get_y_n_input(&query, buf) {
            loop {
                buf.clear();
                println!("Select AI difficulty (1-3): ");
                std::io::stdin()
                    .read_line(buf)
                    .expect("could not read line");
                use input::PlayerType::*;
                match buf.trim().parse::<i32>() {
                    Ok(1) => *player = Player::new(name, Random),
                    Ok(2) => *player = Player::new(name, FindWinning),
                    Ok(3) => *player = Player::new(name, BlockLosing),
                    _ => continue,
                }
                break;
            }
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.p1_turn = true;
        self.done = false;
    }

    pub fn get_input(&mut self, buf: &mut String) -> bool {
        buf.clear();
        let curr = self.curr_player();
        let input = curr.make_move(&self.board, buf);
        if let Err(e) = self.board.set_cell(input, self.curr_player().name()) {
            println!("{}", e.to_string());
            false
        } else {
            true
        }
    }

    // pub fn from_player_info(p1: char, p1_human: bool, p2: char, p2_human: bool) -> Result<Self> {
    //     // Probably not necessary if you validate at time of input but definitely best practice
    //     if p1 == p2 {
    //         Err(anyhow!("You cannot use the same character for both players!"))
    //     } else if p1.is_numeric() || p2.is_numeric() {
    //         Err(anyhow!("Numeric player identifiers are not supported!"))
    //     } else {
    //         Ok(Self { 
    //             board: Grid::from_vec(vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'], 3), 
    //             p1: Player::new(p1, p1_human), 
    //             p2: Player::new(p2, p2_human), 
    //             p1_turn: true,
    //             done: false,
    //             buf: String::new(),
    //         })
    //     }
    // }

    fn curr_player(&self) -> &Player {
        if self.p1_turn {
            &self.p1
        } else {
            &self.p2
        }
    }

    pub fn render(&self) {
        print!("\x1b[H\x1b[J");
        println!("{}", self.board);
    }

    pub fn run(&mut self, buf: &mut String) {
        let mut input_valid;
        while !self.done {
            input_valid = false;
            while !input_valid {
                self.render();
                input_valid = self.get_input(buf);
            }
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
        game.set_player(1, &mut buf).unwrap();
        game.set_player(2, &mut buf).unwrap();
        game.run(&mut buf);
        if !Game::get_y_n_input("Play again?", &mut buf) {
            break;
        }
        game.reset();
    }
}