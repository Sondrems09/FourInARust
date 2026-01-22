pub mod board;
mod engine;
mod human;

use board::{Board, Piece};
use engine::Engine;
use human::Human;

enum Player {
    Human(Human),
    Engine(Engine),
}
impl Agent for Player {
    fn make_move(&self, board: &mut Board, piece: Piece) -> usize {
        match self {
            Player::Human(h) => return h.make_move(board, piece),
            Player::Engine(e) => return e.make_move(board, piece),
        }
    }
}

trait Agent {
    fn make_move(&self, board: &mut Board, piece: Piece) -> usize;
}

pub struct Game {
    x: Player,
    o: Player,
    board: Board,
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

            let col = current.make_move(&mut self.board, piece);

            match self.board.check_win(col) {
                Some(piece) => match piece {
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
                },
                None => (),
            }

            turn = 1 - turn;
        }
    }
}
