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

pub struct Rook {
    color: Color,
    position: Position,
    data: PieceData,
}

impl Rook {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            data: PieceData {
                can_en_passant: false,
                can_double_move: false,
                can_castle: position.file == FileLetter::A || position.file == FileLetter::H,
            },
            color,
            position,
        }
    }
}

impl Piece for Rook {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_type(&self) -> PieceType {
        PieceType::Rook
    }

    fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();

        //top
        for i in 1..8 {
            let mut position = self.position.clone();

            position.rank += i;

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if *piece.get_color() != self.color {
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

        //right
        for i in 1..8 {
            let mut position = self.position.clone();

            position.file += i;

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if *piece.get_color() != self.color {
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

        //bottom
        for i in 1..8 {
            let mut position = self.position.clone();

            position.rank -= i;

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if *piece.get_color() != self.color {
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

        //left
        for i in 1..8 {
            let mut position = self.position.clone();

            position.file -= i;

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if *piece.get_color() != self.color {
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

        moves.retain(|m| !m.to.is_oob() && !m.from.is_oob());

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
