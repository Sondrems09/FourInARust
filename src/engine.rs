use crate::{Agent, board};

pub struct Engine;

struct GameState {
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
    fn make_move(&self, board: &mut board::Board, piece: board::Piece) {
        let game_state = self.minimax(board, 10);
        println!("Eval: {}", game_state.eval);

        board.insert_piece(game_state.best_move, piece).expect("failed to make a move");
    }
}

impl Engine {
    fn minimax(&self, board: &mut board::Board, depth: u32) -> GameState {
        let mut game_state = GameState::new();
        
        if let Some(piece) = board.check_win() {
            match piece {
                board::Piece::O => {
                    game_state.eval = 1_000_000;
                    return game_state; 
                }
                board::Piece::X => {
                    game_state.eval = -1_000_000;
                    return game_state;
                }
                _ => ()
            }
        }

        if board.is_full() || depth == 0 {
            game_state.eval = Engine::eval(board);
            return game_state;
        }
        
        game_state
    }

    #[allow(unused)]
    fn eval(board: &board::Board) -> isize {
        let mut eval: isize = 0;
        0
    }
}
