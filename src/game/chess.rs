use std::error::Error;

use crate::{
    parts::board::Board,
    types::{color::Color, r#move::Move},
};

// contains board and turn handling
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

    pub fn get_turn(&self) -> &Color {
        &self.turn
    }

    pub fn make_move(&mut self, m: Move) -> Result<(), Box<dyn Error>> {
        self.board.make_move(&m)?;

        self.turn = self.turn.get_opposite();

        Ok(())
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
}

impl Default for Chess {
    fn default() -> Self {
        Self::new(Board::new())
    }
}

impl Clone for Chess {
    fn clone(&self) -> Self {
        Self {
            board: self.board.clone(),
            turn: self.turn,
        }
    }
}
