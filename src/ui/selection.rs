use crate::core::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum Selection {
    SelectPiece(Position),
    SelectMove(Vec<Position>, Position),
}
