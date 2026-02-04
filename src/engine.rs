use crate::{Agent, board};

pub struct Engine;

struct GameState {
    eval: isize,
    best_move: usize,
}

#[allow(unused)]
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
        let game_state = self.negmax(&mut board_clone, piece, -100_000_000_000, 100_000_000_000, 10);
        println!("Eval: {}", game_state.eval);

        board
            .insert_piece(game_state.best_move, piece)
            .expect("failed to make a move");

        board.eval = game_state.eval;
    }
}

impl Engine {
    fn negmax(
        &self,
        board: &mut board::Board,
        piece: board::Piece,
        mut alpha: isize,
        beta: isize,
        depth: u32,
    ) -> GameState {

        if let Some(result) = board.is_terminal() {
            match result {
                board::Piece::O => {
                    return GameState {
                        eval: 100_000_000 + depth as isize,
                        best_move: 0,
                    };
                }
                board::Piece::X => {
                    return GameState {
                        eval: -100_000_000 - depth as isize,
                        best_move: 0,
                    };
                }
                board::Piece::Empty => {
                    return GameState {
                        eval: 0,
                        best_move: 0,
                    };
                }
            };
        }

        if depth == 0 {
            return GameState {
                eval: Engine::eval(board, piece),
                best_move: 0,
            };
        }

        let moves = Engine::order_moves(board, piece);

        let mut best_eval = isize::MIN;
        let mut best_move = 0;

        for current_move in moves {
            if board.insert_piece(current_move, piece).is_err() {
                continue;
            }
            let child = self.negmax(board, piece.opponent(), -beta, -alpha, depth - 1);
            let score = -child.eval;

            board.undo_move(current_move);

            if score > best_eval {
                best_eval = score;
                best_move = current_move;
            }

            alpha = alpha.max(best_eval);
            if alpha >= beta {
                break;
            }
        }

        GameState {
            eval: best_eval,
            best_move: best_move,
        }
    }

    pub fn order_moves(board: &mut board::Board, piece: board::Piece) -> Vec<usize> {
        let mut moves = board.get_moves();
        let center = 4;

        moves.sort_by_key(|&col| {
            let mut score = 0;
            if board.creates_three_in_a_row(col, piece) { score += 1000 }
            score += 100 - (col as isize - center as isize).abs() as isize;
            -score
        }
        );

        moves
    }

    pub fn eval(board: &board::Board, piece: board::Piece) -> isize {
        let mut eval: isize = 0;

        if let Some(result) = board.check_win() {
            return match result {
                board::Piece::O => 100_000_000,
                board::Piece::X => 100_000_000,
                board::Piece::Empty => 0,
            };
        }

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
