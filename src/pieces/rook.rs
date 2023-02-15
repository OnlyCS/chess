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
        for i in 1..=8 {
            let position = match self.position.up(i) {
                Ok(p) => p,
                Err(_) => break,
            };

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if *piece.get_color() != self.color {
                        moves.push(Move::new(
                            self.position.clone(),
                            position,
                            vec![MoveModifier::Capture],
                        ));
                    }

                    break;
                } else {
                    moves.push(Move::new(self.position.clone(), position, vec![]));
                }
            } else {
                break;
            }
        }

        //right
        for i in 1..=8 {
            let position = match self.position.right(i) {
                Ok(p) => p,
                Err(_) => break,
            };

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if *piece.get_color() != self.color {
                        moves.push(Move::new(
                            self.position.clone(),
                            position,
                            vec![MoveModifier::Capture],
                        ));
                    }

                    break;
                } else {
                    moves.push(Move::new(self.position.clone(), position, vec![]));
                }
            } else {
                break;
            }
        }

        //bottom
        for i in 1..=8 {
            let position = match self.position.down(i) {
                Ok(p) => p,
                Err(_) => break,
            };

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if *piece.get_color() != self.color {
                        moves.push(Move::new(
                            self.position.clone(),
                            position,
                            vec![MoveModifier::Capture],
                        ));
                    }

                    break;
                } else {
                    moves.push(Move::new(self.position.clone(), position, vec![]));
                }
            } else {
                break;
            }
        }

        //left
        for i in 1..=8 {
            let position = match self.position.left(i) {
                Ok(p) => p,
                Err(_) => break,
            };

            if let Some(square) = board.square(&position) {
                if let Some(piece) = square.get_piece() {
                    if *piece.get_color() != self.color {
                        moves.push(Move::new(
                            self.position.clone(),
                            position,
                            vec![MoveModifier::Capture],
                        ));
                    }

                    break;
                } else {
                    moves.push(Move::new(self.position.clone(), position, vec![]));
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
        if self.data.can_castle {
            self.data.can_castle = false;
        }

        self.position = position;
    }

    fn get_data(&self) -> Option<&PieceData> {
        Some(&self.data)
    }

    fn set_data(&mut self, data: PieceData) {
        self.data = data;
    }
}
