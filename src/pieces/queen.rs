use crate::{
    parts::{board::Board, position::Position},
    types::{color::Color, piece_type::PieceType, r#move::Move},
};

use super::{bishop::Bishop, piece::Piece, rook::Rook};

pub struct Queen {
    color: Color,
    position: Position,
}

impl Queen {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Queen {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_type(&self) -> PieceType {
        PieceType::Queen
    }

    fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();

        moves.extend(Rook::new(self.color, self.position.clone()).get_moves(board));
        moves.extend(Bishop::new(self.color, self.position.clone()).get_moves(board));

        moves
    }

    fn copy(&self) -> Box<dyn Piece> {
        Box::new(Self {
            color: self.color,
            position: self.position.clone(),
        })
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}
