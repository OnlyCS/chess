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
    data: PieceData,
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
            if let Ok(Some(square)) = match self.color {
                Color::White => self.position.up(2).map(|i| board.square(&i)),
                Color::Black => self.position.down(2).map(|i| board.square(&i)),
            } {
                if square.is_empty() {
                    moves.push(Move::new(
                        self.position.clone(),
                        square.get_position().clone(),
                        vec![],
                    ));
                }
            }
        }

        // check a single move
        if let Ok(Some(square)) = match self.color {
            Color::White => self.position.up(1).map(|i| board.square(&i)),
            Color::Black => self.position.down(1).map(|i| board.square(&i)),
        } {
            if square.is_empty() {
                moves.push(Move::new(
                    self.position.clone(),
                    square.get_position().clone(),
                    vec![],
                ));
            }
        }

        // check capture to right
        if let Ok(Ok(Some(square))) = match self.color {
            Color::White => self
                .position
                .up(1)
                .map(|i| i.right(1).map(|j| board.square(&j))),
            Color::Black => self
                .position
                .down(1)
                .map(|i| i.right(1).map(|j| board.square(&j))),
        } {
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
        if let Ok(Ok(Some(square))) = match self.color {
            Color::White => self
                .position
                .up(1)
                .map(|i| i.left(1).map(|j| board.square(&j))),
            Color::Black => self
                .position
                .down(1)
                .map(|i| i.left(1).map(|j| board.square(&j))),
        } {
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
        if let Ok(Some(Some(ep_piece))) = self
            .position
            .left(1)
            .map(|i| board.square(&i).map(|j| j.get_piece()))
        {
            if ep_piece.get_color() != self.get_color()
                && ep_piece.get_type() == PieceType::Pawn
                && ep_piece.get_data().expect("unreachable").can_en_passant
            {
                moves.push(Move::new(
                    self.position.clone(),
                    match self.get_color() {
                        Color::White => ep_piece.get_position().clone().up_loop(1),
                        Color::Black => ep_piece.get_position().clone().down_loop(1),
                    },
                    vec![MoveModifier::EnPassant],
                ));
            }
        }

        // check en-passant exists to the right
        if let Ok(Some(Some(ep_piece))) = self
            .position
            .right(1)
            .map(|i| board.square(&i).map(|j| j.get_piece()))
        {
            if ep_piece.get_color() != self.get_color()
                && ep_piece.get_type() == PieceType::Pawn
                && ep_piece.get_data().expect("unreachable").can_en_passant
            {
                moves.push(Move::new(
                    self.position.clone(),
                    match self.get_color() {
                        Color::White => ep_piece.get_position().clone().up_loop(1),
                        Color::Black => ep_piece.get_position().clone().down_loop(1),
                    },
                    vec![MoveModifier::EnPassant],
                ));
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
        let oldpos = self.position.clone();

        if self.data.can_en_passant {
            self.data.can_en_passant = false;
        }

        if self.data.can_double_move {
            self.data.can_double_move = false;
        }

        if position
            == match self.color {
                Color::White => oldpos.up(2),
                Color::Black => oldpos.down(2),
            }
            .unwrap_or(oldpos)
        {
            self.data.can_en_passant = true;
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
