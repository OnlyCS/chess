use crate::core::{color::Color, piece::PieceType, position::Position};

#[derive(PartialEq, Clone, Debug, Copy, Eq)]
pub enum MoveModifier {
    Capture,
    EnPassant,
    PromotionUnknown(Color),
    Promotion(PieceType),
    CastleKingSide,
    CastleQueenSide,
    PawnDoubleMove,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub modifiers: Vec<MoveModifier>,
    pub piece: PieceType,
    pub color: Color,
}

impl Default for Move {
    fn default() -> Self {
        Self {
            from: Position::default(),
            to: Position::default(),
            modifiers: vec![],
            piece: PieceType::Pawn,
            color: Color::White,
        }
    }
}
