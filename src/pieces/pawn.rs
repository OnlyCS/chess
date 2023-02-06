use crate::{
    parts::{board::Board, position::Position},
    types::{
        color::Color,
        piece_type::PieceType,
        r#move::{Move, MoveModifier},
    },
};

use super::piece::{Piece, PieceData};

pub struct Pawn {
    color: Color,
    position: Position,
    pub data: PieceData,
}

impl Pawn {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            data: PieceData {
                can_en_passant: false,
                can_double_move: match color {
                    Color::White => position.rank == 2,
                    Color::Black => position.rank == 7,
                },
                can_castle: false,
            },
            color,
            position,
        }
    }
}

impl Piece for Pawn {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_type(&self) -> PieceType {
        PieceType::Pawn
    }

    fn get_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();

        // check a double move
        if self.data.can_double_move {
            if let Ok(Some(square)) = self.position.up(2).map(|i| board.square(&i)) {
                if square.is_empty() {
                    moves.push(Move::new(
                        self.position.clone(),
                        Position::new(self.position.clone().file, self.position.rank + 2),
                        vec![],
                    ));
                }
            }
        }

        // check a single move
        if let Ok(Some(square)) = self.position.up(1).map(|i| board.square(&i)) {
            if square.is_empty() {
                moves.push(Move::new(
                    self.position.clone(),
                    square.get_position().clone(),
                    vec![],
                ));
            }
        }

        // check capture to right
        if let Ok(Ok(Some(square))) = self
            .position
            .up(1)
            .map(|i| i.right(1).map(|j| board.square(&j)))
        {
            if let Some(piece) = square.get_piece() {
                if piece.get_color() != self.get_color() {
                    moves.push(Move::new(
                        self.position.clone(),
                        square.get_position().clone(),
                        vec![],
                    ));
                }
            }
        }

        // check capture to left
        if let Ok(Ok(Some(square))) = self
            .position
            .up(1)
            .map(|i| i.left(1).map(|j| board.square(&j)))
        {
            if let Some(piece) = square.get_piece() {
                if piece.get_color() != self.get_color() {
                    moves.push(Move::new(
                        self.position.clone(),
                        square.get_position().clone(),
                        vec![],
                    ));
                }
            }
        }

        // check en-passant exists to the left
        if let Ok(Some(ep_square)) = self.position.left(1).map(|i| board.square(&i)) {
            if let Some(ep_piece) = ep_square.get_piece() {
                if ep_piece.get_color() != self.get_color()
                    && ep_piece.get_type() == PieceType::Pawn
                    && ep_piece.get_data().expect("unreachable").can_en_passant
                {
                    moves.push(Move::new(
                        self.position.clone(),
                        ep_square.get_position().clone(),
                        vec![MoveModifier::EnPassant],
                    ));
                }
            }
        }

        // check en-passant exists to the right
        if let Ok(Some(ep_square)) = self.position.right(1).map(|i| board.square(&i)) {
            if let Some(ep_piece) = ep_square.get_piece() {
                if ep_piece.get_color() != self.get_color()
                    && ep_piece.get_type() == PieceType::Pawn
                    && ep_piece.get_data().expect("unreachable").can_en_passant
                {
                    moves.push(Move::new(
                        self.position.clone(),
                        ep_square.get_position().clone(),
                        vec![MoveModifier::EnPassant],
                    ));
                }
            }
        }

        for m in moves.iter_mut() {
            if m.to.rank == 8 && self.color == Color::White
                || m.to.rank == 1 && self.color == Color::Black
            {
                m.modifiers.push(MoveModifier::Promotion);
            }
        }

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
