use crate::parts::{board::Board, position::Position};

use super::color::Color;

#[derive(PartialEq, Clone)]
pub enum MoveModifier {
    Capture,
    EnPassant,
    Promotion,
    CastleKingSide,
    CastleQueenSide,
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
        self.retain(|mv| {
            let mut working_board = board.clone();
            match working_board.make_move(mv) {
                Ok(_) => {}
                Err(_) => return false,
            };

            // check the number of kings on board
            !working_board.is_check(color)
        });
    }
}
