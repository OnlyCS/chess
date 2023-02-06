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
        let move_nums = vec![
            self.position.up(1).map(|i| i.right(1)),
            self.position.up(1).map(|i| i.left(1)),
            self.position.down(1).map(|i| i.right(1)),
            self.position.down(1).map(|i| i.left(1)),
            Ok(self.position.up(1)),
            Ok(self.position.down(1)),
            Ok(self.position.right(1)),
            Ok(self.position.left(1)),
        ];

        let mut moves = Vec::new();

        for position in move_nums.into_iter().flatten().flatten() {
            let mut capture = false;
            let mut keep = true;

            if let Some(Some(piece)) = board.square(&position).map(|x| x.get_piece()) {
                if piece.get_color() != &self.color {
                    capture = true;
                } else {
                    keep = false;
                }
            }

            if keep {
                let mut modifiers = Vec::new();
                if capture {
                    modifiers.push(MoveModifier::Capture);
                }

                moves.push(Move::new(self.position.clone(), position, modifiers));
            }
        }

        moves.retain(|m| !m.from.is_oob() && !m.to.is_oob());

        moves
    }

    fn copy(&self) -> Box<dyn Piece + Sync + Send> {
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
