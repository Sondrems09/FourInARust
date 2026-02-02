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
        let game_state = self.minimax(&mut board_clone, piece, 7);
        println!("Eval: {}", game_state.eval);

        board
            .insert_piece(game_state.best_move, piece)
            .expect("failed to make a move");

        board.eval = game_state.eval;
    }
}

impl Engine {
    fn minimax(&self, board: &mut board::Board, piece: board::Piece, depth: u32) -> GameState {
        if let Some(piece) = board.check_win() {
            match piece {
                board::Piece::O => {
                    let game_state = GameState { eval: 1_000_000_000, best_move: 0 };
                    return game_state;
                }
                board::Piece::X => {
                    let game_state = GameState { eval: -1_000_000_000, best_move: 0};
                    return game_state;
                }
                _ => (),
            }
        }

        if board.is_full() || depth == 0 {
            let game_state = GameState { eval: Engine::eval(board), best_move: 0};
            return game_state;
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
                let result = self.minimax(board, board::Piece::O, depth-1);
                current_test_play.eval = result.eval;
            } else {
                let result = self.minimax(board, board::Piece::X, depth-1);
                current_test_play.eval = result.eval
            }
            
            board.undo_move(i);

            all_plays.push(current_test_play);
        };

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

    fn eval(board: &board::Board) -> isize {
        let mut eval: isize = 0;

        let (diagonals_up, diagonals_down) = board.diagonals();
        let cols = board.cols();
        let rows = board.rows();

        let lines_collections = vec![diagonals_up, diagonals_down, cols, rows];

        for lines in lines_collections {
            for line in lines {
                for i in 1..4 {
                    for (n, window) in line.windows(i).enumerate() {
                        if window.iter().all(|&x| x == board::Piece::O) {
                            let after = if n + i < line.len() { line[n + i] } else { board::Piece::X };
                            let before = if n > 0 { line[n - 1] } else { board::Piece::X };
                            
                            if after == board::Piece::X || before == board::Piece::X {
                                if after == board::Piece::X && before == board::Piece::X {
                                    eval += 0;
                                } else { 
                                    eval += (10_isize.pow(i as u32))/2
                                }
                            } else {
                                eval += 10_isize.pow(i as u32);
                            }
                        }
                        if window.iter().all(|&x| x == board::Piece::X) {
                            let after = if n + i < line.len() { line[n + i] } else { board::Piece::O };
                            let before = if n > 0 { line[n - 1] } else { board::Piece::O };
                            
                            if after == board::Piece::O || before == board::Piece::O {
                                if after == board::Piece::O && before == board::Piece::O {
                                    eval -= 0;
                                } else {
                                    eval -= (10_isize.pow(i as u32))/2
                                }
                            } else {
                                eval -= 10_isize.pow(i as u32);
                            }
                        }
                    }
                }
            }
        }

        eval
    }
}
