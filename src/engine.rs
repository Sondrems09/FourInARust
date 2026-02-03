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
        let mut board_clone = board.clone();
        println!("Chosing a move...");
        let game_state = self.negmax(&mut board_clone, piece, 6);
        println!("Eval: {}", game_state.eval);

        board
            .insert_piece(game_state.best_move, piece)
            .expect("failed to make a move");

        board.eval = game_state.eval;
    }
}

impl Engine {
    fn negmax(&self, board: &mut board::Board, piece: board::Piece, depth: u32) -> GameState {
        if let Some(result) = board.is_terminal() {
            match result {
                board::Piece::O => return GameState { eval: 100_000_000, best_move: 0 },
                board::Piece::X => return GameState { eval: -100_000_000, best_move: 0 },
                board::Piece::Empty => return GameState { eval: 0, best_move: 0 },
            };
        }       

        if depth == 0 {
            return GameState { eval: Engine::eval(board, piece), best_move: 0 };
        }

        // Get index of all non-full columns (possible moves)
        let mut empty_cols = Vec::new();
        let cols = board.cols();
        for i in 0..cols.len() {
            if cols[i].contains(&board::Piece::Empty) {
                empty_cols.push(i);
            }
        }

        let mut all_plays: Vec<GameState> = Vec::new();

        for i in empty_cols {
            let mut current_test_play = GameState::new();
            current_test_play.best_move = i;

            if board.insert_piece(i, piece).is_err() {
                continue;
            }

            if piece == board::Piece::X {
                let result = self.negmax(board, board::Piece::O, depth - 1);
                current_test_play.eval = result.eval;
            } else {
                let result = self.negmax(board, board::Piece::X, depth - 1);
                current_test_play.eval = result.eval
            }

            board.undo_move(i);

            all_plays.push(current_test_play);
        }

        let mut best_play = GameState::new();

        if piece == board::Piece::O {
            let mut best_eval = isize::MIN;
            for play in all_plays {
                if play.eval > best_eval {
                    best_eval = play.eval;
                    best_play = play;
                }
            }
        } else {
            let mut best_eval = isize::MAX;
            for play in all_plays {
                if play.eval < best_eval {
                    best_eval = play.eval;
                    best_play = play;
                }
            }
        }
        best_play
    }

    pub fn eval(board: &board::Board, piece: board::Piece) -> isize {
        let mut eval: isize = 0;

        let (diagonals_up, diagonals_down) = board.diagonals();
        let cols = board.cols();
        let rows = board.rows();

        let lines = diagonals_up
            .into_iter()
            .chain(diagonals_down)
            .chain(cols)
            .chain(rows);

        for line in lines {
            match piece {
                board::Piece::O => eval += Engine::evaluate_line(&line, piece),
                board::Piece::X => eval -= Engine::evaluate_line(&line, piece),
                _ => panic!("Invalid player"),
            }
        }

        eval
    }

    fn evaluate_line(line: &[board::Piece], piece: board::Piece) -> isize {
        let mut eval: isize = 0;

        for window in line.windows(4) {
            let count_pieces = window.iter().filter(|&&p| p == piece).count();
            let count_empty = window.iter().filter(|&&p| p == board::Piece::Empty).count();

            if count_pieces + count_empty == 4 {
                // window has only your own pieces and empty spaces
                // evaled exponentially with how many pieces you have filled
                eval += 10_isize.pow(count_pieces as u32)
            } else {
                // window is blocked, evaled to 0
                eval += 0;
                continue;
            }
        }

        eval
    }
}
