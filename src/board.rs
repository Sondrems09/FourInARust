#[derive(Clone, Debug)]
pub struct Board {
    cols: [[Piece; 6]; 7],
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Piece {
    X,
    O,
    Empty,
}

impl Board {
    pub fn new() -> Board {
        Board {
            cols: [[Piece::Empty; 6]; 7],
        }
    }

    pub fn insert_piece(&mut self, col: usize, piece: Piece) -> Result<(), &'static str> {
        // Recursiveley call the function again if the column is full
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

        Ok(())
    }

    pub fn check_win(&self, col: usize) -> Option<Piece> {
        // Check the column for four in a row
        for window in self.cols[col].windows(4) {
            if window.iter().all(|&x| x == Piece::X) {
                return Some(Piece::X);
            } else if window.iter().all(|&x| x == Piece::O) {
                return Some(Piece::O);
            }
        }

        // Find the x and y of the last piece inserted
        let mut last_piece_y = 5;
        while self.cols[col][last_piece_y] == Piece::Empty {
            last_piece_y -= 1;
        }
        let last_piece_x = col;

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

        let (diagonal1, diagonal2) = self.get_diagonals(last_piece_x, last_piece_y);

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

    fn get_diagonals(&self, last_x: usize, last_y: usize) -> (Vec<Piece>, Vec<Piece>) {
        let x_right = 6 - last_x; // Six starting from 0 is the lenght of the row
        let x_left = 6 - x_right;

        let y_above = 5 - last_y; // Same principal as above, five is the legth of each column
        let y_below = 5 - y_above;

        // Getting the first diagonal
        // Find the smallest value to the "left" and the smallest value to the "right"
        let smallest_left = x_left.min(y_below);
        let smallest_right = x_right.min(y_above);

        // Make a square
        let side_len = smallest_left + smallest_right + 1;

        // Find the starting x and y
        let mut x = last_x - smallest_left;
        let mut y = last_y - smallest_left;

        // Find the values
        let mut diagonal1 = Vec::new();
        for _ in 0..side_len {
            diagonal1.push(self.cols[x][y]);
            x += 1;
            y += 1;
        }

        // Get the second diagonal
        // Here we need to do some shifting around of variables
        let smallest_left = x_left.min(y_above);
        let smallest_right = x_right.min(y_below);

        // Make a square
        let side_len = smallest_left + smallest_right + 1;

        // Find the starting x and y
        let mut x = last_x - smallest_left;
        let mut y = last_y + smallest_left;

        // Find the values
        let mut diagonal2 = Vec::new();
        for _ in 0..side_len {
            diagonal2.push(self.cols[x][y]);
            x += 1;
            y = y.saturating_sub(1); // Saturating sub to avoid an underflow error
        }

        (diagonal1, diagonal2)
    }

    pub fn display(&self) {
        clearscreen::clear().expect("failed to clear screen");

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
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn insert_piece_works() {
        let mut board = Board::new();

        board.insert_piece(0, Piece::O);

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
        };

        let diagonal1 = board.get_diagonals(0, 0);

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
        };

        let win = board.check_win(0);
        assert_eq!(win, Some(Piece::X));
    }
}
