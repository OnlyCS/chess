pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty,
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
