pub mod board;
mod engine;
mod human;

use board::{Board, Piece};
use engine::Engine;
use human::Human;

#[allow(unused)]
enum Player {
    Human(Human),
    Engine(Engine),
}

impl Agent for Player {
    fn make_move(&self, board: &mut Board, piece: Piece) {
        match self {
            Player::Human(h) => h.make_move(board, piece),
            Player::Engine(e) => e.make_move(board, piece),
        }
    }
}

trait Agent {
    fn make_move(&self, board: &mut Board, piece: Piece);
}

pub struct Game {
    x: Player,
    o: Player,
    board: Board,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            x: Player::Human(Human),
            o: Player::Human(Human),
            board: Board::new(),
        }
    }
    pub fn run(&mut self) {
        let mut turn = 0;
        loop {
            self.board.display();

            let piece = if turn == 0 { Piece::O } else { Piece::X };
            let current = if piece == Piece::O { &self.o } else { &self.x };

            current.make_move(&mut self.board, piece);

            if self.board.is_full() {
                self.board.display();
                println!("Board is full, its a draw");
                break;
            }

            if let Some(piece) = self.board.check_win() {
                match piece {
                    Piece::X => {
                        self.board.display();
                        println!("X won!");
                        break;
                    }
                    Piece::O => {
                        self.board.display();
                        println!("O won!");
                        break;
                    }
                    _ => panic!("Invalid winner"),
                }
            }

            turn = 1 - turn;
        }
    }
}
