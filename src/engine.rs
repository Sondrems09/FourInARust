use crate::{Agent, board};

pub struct Engine;

pub struct GameState {
    eval: isize,
    best_move: usize,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            eval: 0,
            best_move: 0,
        }
    }
}

impl Agent for Engine {
    fn make_move(&self, board: &mut board::Board, piece: board::Piece) -> usize {
        0
    }
}

impl Engine {
    pub fn minimax(&self, board: &mut board::Board) -> GameState {
        let game_state = GameState::new();

        game_state
    }

    fn eval(board: &board::Board) -> isize {
        0
    }

    fn get_diagonals() {}
}
