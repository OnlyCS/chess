use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn get_value(&self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 100,
        }
    }
}

impl PartialOrd for PieceType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_value().cmp(&other.get_value()))
    }
}

impl PartialEq for PieceType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (PieceType::Pawn, PieceType::Pawn)
                | (PieceType::Knight, PieceType::Knight)
                | (PieceType::Bishop, PieceType::Bishop)
                | (PieceType::Rook, PieceType::Rook)
                | (PieceType::Queen, PieceType::Queen)
                | (PieceType::King, PieceType::King)
        )
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "P"),
            PieceType::Knight => write!(f, "Kn"),
            PieceType::Bishop => write!(f, "B"),
            PieceType::Rook => write!(f, "R"),
            PieceType::Queen => write!(f, "Q"),
            PieceType::King => write!(f, "K"),
        }
    }
}
