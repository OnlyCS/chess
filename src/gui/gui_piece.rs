use core::fmt;

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn every() -> impl Iterator<Item = PieceType> {
        [
            PieceType::Pawn,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ]
        .into_iter()
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "pawn"),
            PieceType::Knight => write!(f, "knight"),
            PieceType::Bishop => write!(f, "bishop"),
            PieceType::Rook => write!(f, "rook"),
            PieceType::Queen => write!(f, "queen"),
            PieceType::King => write!(f, "king"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceType,
}

impl Piece {
    pub fn image(&self) -> String {
        format!("file://assets/{}_{}.svg", self.kind, self.color)
    }
}
