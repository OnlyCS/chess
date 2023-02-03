use crate::{
    parts::board::Board,
    types::{color::Color, r#move::Move},
};

pub struct Chess {
    board: Board,
    turn: Color,
}

impl Chess {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            turn: Color::White,
        }
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_moves(&self) -> Vec<Move> {
        self.board.get_moves_for(self.turn)
    }
}

impl Default for Chess {
    fn default() -> Self {
        Self::new(Board::new())
    }
}