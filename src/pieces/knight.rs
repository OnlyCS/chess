use crate::{
    parts::{board::Board, position::Position},
    types::{
        color::Color,
        piece_type::PieceType,
        r#move::{Move, MoveModifier},
    },
};

use super::piece::Piece;

pub struct Knight {
    color: Color,
    position: Position,
}

impl Knight {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Knight {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_type(&self) -> PieceType {
        PieceType::Knight
    }

    fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = vec![
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file + 1, self.position.rank + 2),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file + 1, self.position.rank - 2),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file - 1, self.position.rank + 2),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file - 1, self.position.rank - 2),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file + 2, self.position.rank + 1),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file + 2, self.position.rank - 1),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file - 2, self.position.rank + 1),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file - 2, self.position.rank - 1),
                None,
            ),
        ];

        let mut remove = Vec::new();
        for (i, m) in moves.iter_mut().enumerate() {
            if let Some(square) = board.square(&m.to) {
                if let Some(piece) = square.get_piece() {
                    if piece.get_color() == &self.color {
                        remove.push(i);
                    } else {
                        m.modifiers = Some(vec![MoveModifier::Capture]);
                    }
                }
            } else {
                remove.push(i);
            }
        }

        for i in remove.iter().rev() {
            moves.remove(*i);
        }

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
