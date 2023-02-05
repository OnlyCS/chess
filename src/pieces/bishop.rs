use crate::{
    parts::{board::Board, position::Position},
    types::{
        color::Color,
        piece_type::PieceType,
        r#move::{Move, MoveFilter, MoveModifier},
    },
};

use super::piece::Piece;

pub struct Bishop {
    color: Color,
    position: Position,
}

impl Bishop {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Bishop {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_type(&self) -> PieceType {
        PieceType::Bishop
    }

    fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();

        // top-right
        for i in 1..=8 {
            let mut position = self.position.clone();

            position.file += i;
            position.rank += i;

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if piece.get_color() != &self.color {
                        moves.push(Move::new(
                            self.position.clone(),
                            position,
                            Some(vec![MoveModifier::Capture]),
                        ));
                    }

                    break;
                } else {
                    moves.push(Move::new(self.position.clone(), position, None));
                }
            } else {
                break;
            }
        }

        // top-left
        for i in 1..=8 {
            let mut position = self.position.clone();

            position.file -= i;
            position.rank += i;

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if piece.get_color() != &self.color {
                        moves.push(Move::new(
                            self.position.clone(),
                            position,
                            Some(vec![MoveModifier::Capture]),
                        ));
                    }

                    break;
                } else {
                    moves.push(Move::new(self.position.clone(), position, None));
                }
            } else {
                break;
            }
        }

        // bottom-right
        for i in 1..=8 {
            let mut position = self.position.clone();

            position.file += i;
            position.rank -= i;

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if piece.get_color() != &self.color {
                        moves.push(Move::new(
                            self.position.clone(),
                            position,
                            Some(vec![MoveModifier::Capture]),
                        ));
                    }

                    break;
                } else {
                    moves.push(Move::new(self.position.clone(), position, None));
                }
            } else {
                break;
            }
        }

        // bottom-left
        for i in 1..=8 {
            let mut position = self.position.clone();

            position.file -= i;
            position.rank -= i;

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if piece.get_color() != &self.color {
                        moves.push(Move::new(
                            self.position.clone(),
                            position,
                            Some(vec![MoveModifier::Capture]),
                        ));
                    }

                    break;
                } else {
                    moves.push(Move::new(self.position.clone(), position, None));
                }
            } else {
                break;
            }
        }

        moves.retain(|mv| !mv.to.is_oob() && !mv.from.is_oob());
        moves.filter_king_check(board, self.color);

        moves
    }

    fn copy(&self) -> Box<dyn Piece + Sync + Send> {
        Box::new(Self {
            color: self.color,
            position: self.position.clone(),
        })
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}
