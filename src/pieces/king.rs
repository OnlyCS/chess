use crate::{
    parts::{board::Board, position::Position},
    types::{
        color::Color,
        file_letter::FileLetter,
        piece_type::PieceType,
        r#move::{Move, MoveModifier},
    },
};

use super::piece::{Piece, PieceData};

pub struct King {
    color: Color,
    position: Position,
    data: PieceData,
}

impl King {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            data: PieceData {
                can_en_passant: false,
                can_double_move: false,
                can_castle: match color {
                    Color::White => position == Position::new(FileLetter::E, 1),
                    Color::Black => position == Position::new(FileLetter::E, 8),
                },
            },
            color,
            position,
        }
    }
}

impl Piece for King {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_type(&self) -> PieceType {
        PieceType::King
    }

    fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = vec![
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file + 1, self.position.rank + 1),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file + 1, self.position.rank),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file + 1, self.position.rank - 1),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file, self.position.rank + 1),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file, self.position.rank - 1),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file - 1, self.position.rank + 1),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file - 1, self.position.rank),
                None,
            ),
            Move::new(
                self.position.clone(),
                Position::new(self.position.clone().file - 1, self.position.rank - 1),
                None,
            ),
        ];

        let mut remove = Vec::new();
        for (i, m) in moves.iter_mut().enumerate() {
            if let Some(square) = board.square(&m.to) {
                if let Some(piece) = square.get_piece() {
                    if *piece.get_color() == self.color {
                        remove.push(i)
                    } else {
                        m.modifiers
                            .as_mut()
                            .unwrap_or(&mut Vec::new())
                            .push(MoveModifier::Capture);
                    }
                }
            } else {
                remove.push(i)
            }
        }

        for i in remove.iter().rev() {
            moves.remove(*i);
        }

        moves.retain(|m| !m.from.is_oob() && !m.to.is_oob());

        moves
    }

    fn copy(&self) -> Box<dyn Piece> {
        Box::new(Self {
            color: self.color,
            position: self.position.clone(),
            data: self.data.clone(),
        })
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}
