use std::cmp::Ordering;

pub enum PieceType {
    Empty,
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
            PieceType::Empty => 0,
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
                | (PieceType::Empty, PieceType::Empty)
        )
    }
}
