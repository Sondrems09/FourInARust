use crate::{Agent, board};

pub struct Engine;

impl Agent for Engine {
    fn make_move(&self, board: &mut board::Board, piece: board::Piece) -> usize {
        0
    }
}
