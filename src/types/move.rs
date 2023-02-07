use crate::parts::{board::Board, position::Position};

use super::{color::Color, piece_type::PieceType};

#[derive(PartialEq, Clone)]
pub enum MoveModifier {
    Capture,
    EnPassant,
    Promotion,
    Castle,
}

pub struct Move {
    pub from: Position,
    pub to: Position,
    pub modifiers: Vec<MoveModifier>,
}

impl Move {
    pub fn new(from: Position, to: Position, modifiers: Vec<MoveModifier>) -> Move {
        Move {
            from,
            to,
            modifiers,
        }
    }
}

impl Clone for Move {
    fn clone(&self) -> Self {
        Move {
            from: self.from.clone(),
            to: self.to.clone(),
            modifiers: self.modifiers.clone(),
        }
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to && self.modifiers == other.modifiers
    }
}

impl Default for Move {
    fn default() -> Self {
        Self::new(Position::default(), Position::default(), vec![])
    }
}

pub trait MoveFilter {
    fn filter_king_check(&mut self, board: &Board, color: Color);
}

impl MoveFilter for Vec<Move> {
    fn filter_king_check(&mut self, board: &Board, color: Color) {
        self.retain(|_| {
            let mut king_in_check = false;

            for othermv in board.get_moves_for(color.other()) {
                let mut this_board = board.clone();
                this_board
                    .make_move(&othermv)
                    .expect("Failed to check for king check");

                let is_king = this_board
                    .get_squares()
                    .iter()
                    .any(|sq| {
                        if let Some(piece) = sq.get_piece() {
                            piece.get_type() == PieceType::King && *piece.get_color() == color
                        } else {
                            false
                        }
                    });

                if !is_king {
                    king_in_check = true;
                    break;
                }
            }

            !king_in_check
        });
    }
}
