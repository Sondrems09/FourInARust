#[derive(Clone, Debug)]
pub struct Board {
    cols: [[Piece; 6]; 7],
    pub last_move: usize,
    pub eval: isize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    X,
    O,
    Empty,
}

impl Piece {
    pub fn opponent(&self) -> Piece {
        match self {
            Piece::O => Piece::X,
            Piece::X => Piece::O,
            Piece::Empty => Piece::Empty,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            cols: [[Piece::Empty; 6]; 7],
            last_move: 0,
            eval: 0,
        }
    }

    pub fn insert_piece(&mut self, col: usize, piece: Piece) -> Result<(), &'static str> {
        if !self.cols[col].contains(&Piece::Empty) {
            return Err("Column is full");
        }

        let current_col = &mut self.cols[col];
        let mut i = 5;
        current_col[i] = piece;

        // Move the piece to the next space if it is empty, simulating gravity
        while i > 0 && current_col[i - 1] == Piece::Empty {
            current_col.swap(i, i - 1);
            i -= 1;
        }

        self.last_move = col;

        Ok(())
    }

    pub fn undo_move(&mut self, col: usize) {
        for row in (0..self.cols[col].len()).rev() {
            if self.cols[col][row] != Piece::Empty {
                self.cols[col][row] = Piece::Empty;
                break;
            }
        }
    }

    pub fn check_win(&self) -> Option<Piece> {
        // Check the column for four in a row
        for window in self.cols[self.last_move].windows(4) {
            if window.iter().all(|&x| x == Piece::X) {
                return Some(Piece::X);
            } else if window.iter().all(|&x| x == Piece::O) {
                return Some(Piece::O);
            }
        }

        // Find the x and y of the last piece inserted
        let mut last_piece_y = 5;
        while self.cols[self.last_move][last_piece_y] == Piece::Empty && last_piece_y != 0 {
            last_piece_y = last_piece_y.saturating_sub(1);
        }
        let last_piece_x = self.last_move;

        // Get the three pieces on each side of the last piece inserted
        let mut row: Vec<Piece> = Vec::new();

        let first: usize = last_piece_x.saturating_sub(3);
        let last = (last_piece_x + 3).min(self.cols.len() - 1);

        for x in first..=last {
            row.push(self.cols[x][last_piece_y]);
        }

        for window in row.windows(4) {
            if window.iter().all(|&x| x == Piece::X) {
                return Some(Piece::X);
            } else if window.iter().all(|&x| x == Piece::O) {
                return Some(Piece::O);
            }
        }

        let (diagonal1, diagonal2) = self.get_diagonals_of_last_piece(last_piece_x, last_piece_y);

        if diagonal1.len() >= 4 {
            for window in diagonal1.windows(4) {
                if window.iter().all(|&x| x == Piece::X) {
                    return Some(Piece::X);
                } else if window.iter().all(|&x| x == Piece::O) {
                    return Some(Piece::O);
                }
            }
        }
        if diagonal2.len() >= 4 {
            for window in diagonal2.windows(4) {
                if window.iter().all(|&x| x == Piece::X) {
                    return Some(Piece::X);
                } else if window.iter().all(|&x| x == Piece::O) {
                    return Some(Piece::O);
                }
            }
        }

        None
    }


    pub fn creates_three_in_a_row(&mut self, col: usize,  piece: Piece) -> bool {
        self.insert_piece(col, piece).ok();

        // Check the column for four in a row
        for window in self.cols[self.last_move].windows(4) {
            if window.iter().filter(|&&x| x == piece ).count() == 3 && window.iter().filter(|&&x| x == Piece::Empty).count() == 1 {
                self.undo_move(col);
                return true
            }
        }

        // Find the x and y of the last piece inserted
        let mut last_piece_y = 5;
        while self.cols[self.last_move][last_piece_y] == Piece::Empty && last_piece_y != 0 {
            last_piece_y = last_piece_y.saturating_sub(1);
        }
        let last_piece_x = self.last_move;

        // Get the three pieces on each side of the last piece inserted
        let mut row: Vec<Piece> = Vec::new();

        let first: usize = last_piece_x.saturating_sub(3);
        let last = (last_piece_x + 3).min(self.cols.len() - 1);

        for x in first..=last {
            row.push(self.cols[x][last_piece_y]);
        }

        for window in row.windows(4) {
            if window.iter().filter(|&&x| x == piece ).count() == 3 && window.iter().filter(|&&x| x == Piece::Empty).count() == 1 {
                self.undo_move(col);
                return true
            }
        }

        let (diagonal1, diagonal2) = self.get_diagonals_of_last_piece(last_piece_x, last_piece_y);

        if diagonal1.len() >= 4 {
            for window in diagonal1.windows(4) {
                if window.iter().filter(|&&x| x == piece ).count() == 3 && window.iter().filter(|&&x| x == Piece::Empty).count() == 1 {
                    self.undo_move(col);
                    return true
                }
            }
        }
        if diagonal2.len() >= 4 {
            for window in diagonal2.windows(4) {
                if window.iter().filter(|&&x| x == piece ).count() == 3 && window.iter().filter(|&&x| x == Piece::Empty).count() == 1 {
                    self.undo_move(col);
                    return true
                }
            }
        }

        self.undo_move(col);

        false
    }

    pub fn is_terminal(&self) -> Option<Piece> {
        if let Some(winner) = self.check_win() {
            Some(winner)
        } else if self.is_full() {
            Some(Piece::Empty)
        } else {
            None
        }
    }

    fn get_diagonals_of_last_piece(
        &self,
        last_x: usize,
        last_y: usize,
    ) -> (Vec<Piece>, Vec<Piece>) {
        let cols = self.cols.len();
        let rows = self.cols[0].len();

        // Diagonal /
        let mut diag1 = Vec::new();
        // Step backwards (down-left)
        let mut x = last_x as isize;
        let mut y = last_y as isize;
        while x > 0 && y > 0 {
            x -= 1;
            y -= 1;
        }
        // Step forward (up-right)
        while x < cols as isize && y < rows as isize {
            diag1.push(self.cols[x as usize][y as usize]);
            x += 1;
            y += 1;
        }

        // Diagonal \
        let mut diag2 = Vec::new();
        // Step backwards (up-left)
        let mut x = last_x as isize;
        let mut y = last_y as isize;
        while x > 0 && y < (rows as isize - 1) {
            x -= 1;
            y += 1;
        }
        // Step forward (up-right)
        while x < cols as isize && y >= 0 {
            diag2.push(self.cols[x as usize][y as usize]);
            x += 1;
            y -= 1;
        }

        (diag1, diag2)
    }

    pub fn display(&self) {
        clearscreen::clear().expect("failed to clear screen");

        println!("Eval: {}", self.eval);
        println!();

        for x in 1..=7 {
            print!("{x} ");
        }
        println!();
        for y in (0..6).rev() {
            for x in 0..7 {
                match self.cols[x][y] {
                    Piece::X => print!("\x1b[31mX\x1b[0m "), // Prints 'X' with red text
                    Piece::O => print!("\x1b[33mO\x1b[0m "), // Prints 'O' with yellow text
                    Piece::Empty => print!("\x1b[34m#\x1b[0m "), // prints '#' with blue text
                }
            }
            println!()
        }
        println!();
    }

    pub fn diagonals(&self) -> (Vec<Vec<Piece>>, Vec<Vec<Piece>>) {
        let cols = self.cols.len();
        let rows = self.cols[0].len();

        // Diagonals /
        let mut diagonals_up = Vec::new();

        for i in 0..cols - 1 {
            diagonals_up.push(Vec::new());
            let mut x = i;
            let mut y = 2;

            // Walk back (down-left)
            while x > 0 && y > 0 {
                x -= 1;
                y -= 1;
            }

            // Walk forwards (up-right)
            while x < cols && y < rows {
                diagonals_up[i].push(self.cols[x][y]);
                x += 1;
                y += 1;
            }
        }

        // Diagonals \
        let mut diagonals_down = Vec::new();

        for i in 0..cols - 1 {
            diagonals_down.push(Vec::new());

            let mut x = i as isize;
            let mut y = 3;

            // Walk back (up-left)
            while x > 0 && y < (rows as isize) - 1 {
                x -= 1;
                y += 1;
            }

            // Walk forward (down-right)
            while x < cols as isize && y >= 0 {
                diagonals_down[i].push(self.cols[x as usize][y as usize]);
                x += 1;
                y -= 1;
            }
        }

        (diagonals_up, diagonals_down)
    }

    pub fn rows(&self) -> Vec<Vec<Piece>> {
        let mut rows = Vec::new();

        for y in 0..6 {
            rows.push(Vec::new());
            for x in 0..7 {
                rows[y].push(self.cols[x][y]);
            }
        }

        rows
    }

    pub fn cols(&self) -> Vec<Vec<Piece>> {
        let mut cols = Vec::new();

        for x in 0..7 {
            cols.push(Vec::new());
            for y in 0..6 {
                cols[x].push(self.cols[x][y]);
            }
        }

        cols
    }
    pub fn is_full(&self) -> bool {
        for col in self.cols() {
            for cell in col {
                match cell {
                    Piece::Empty => return false,
                    _ => continue,
                }
            }
        }
        true
    }

    pub fn get_moves(&self) -> Vec<usize> {
        let mut empty_cols = Vec::new();
        let cols = self.cols();
        for i in 0..cols.len() {
            if cols[i].contains(&Piece::Empty) {
                empty_cols.push(i);
            };
        }

        empty_cols
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn insert_piece_works() {
        let mut board = Board::new();

        board.insert_piece(0, Piece::O).unwrap();

        let board2 = Board {
            cols: [
                [
                    Piece::O,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
            ],
            last_move: 0,
            eval: 0,
        };

        assert_eq!(board.cols, board2.cols);
    }

    #[test]
    fn diagonals_work() {
        let board = Board {
            cols: [
                [
                    Piece::O,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::O,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::X,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::O,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
            ],
            last_move: 0,
            eval: 0,
        };

        let diagonal1 = board.get_diagonals_of_last_piece(0, 0);

        assert_eq!(
            diagonal1,
            (
                vec![
                    Piece::O,
                    Piece::O,
                    Piece::X,
                    Piece::Empty,
                    Piece::O,
                    Piece::Empty
                ],
                vec![Piece::O]
            )
        );
    }

    #[test]
    fn win_detection() {
        let board = Board {
            cols: [
                [
                    Piece::X,
                    Piece::X,
                    Piece::X,
                    Piece::X,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::O,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::O,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::O,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::X,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
                [
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                    Piece::Empty,
                ],
            ],
            last_move: 0,
            eval: 0,
        };

        let win = board.check_win();
        assert_eq!(win, Some(Piece::X));
    }
}
