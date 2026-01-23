use std::io;

use crate::{Agent, board};

pub struct Human;

impl Agent for Human {
    fn make_move(&self, board: &mut board::Board, piece: board::Piece) {
        loop {
            let col = Human::input("Enter the number of the column you want to insert a piece in");

            match board.insert_piece(col, piece) {
                Ok(_) => break,
                Err(e) => println!("{e}"),
            }
        }
    }
}

impl Human {
    pub fn input(s: &str) -> usize {
        println!("{s}");

        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim().parse::<usize>() {
                Ok(x) => {
                    if (1..=7).contains(&x) {
                        return x - 1;
                    } else {
                        println!("Please input a number from 1 to 7");
                    }
                }
                Err(_) => println!("Please input a valid number bigger than 0"),
            }
        }
    }
}
