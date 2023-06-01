use serde::{Deserialize, Serialize};

use crate::core::{color::Color, piece::PieceType, position::Position};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Copy, Eq)]
pub enum MoveModifier {
    Capture,
    EnPassant,
    PromotionUnknown(Color),
    Promotion(PieceType),
    CastleKingSide,
    CastleQueenSide,
    PawnDoubleMove,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl Move {
    pub fn to_ard(&self) -> String {
        let fromf: u8 = self.from.file.into();
        let fromr = self.from.rank;
        let tof: u8 = self.to.file.into();
        let tor = self.to.rank;

        let istake = self
            .modifiers
            .iter()
            .filter(|m| {
                let modifier = **m;

                matches!(
                    modifier,
                    MoveModifier::Capture
                        | MoveModifier::Promotion(_)
                        | MoveModifier::PromotionUnknown(_)
                )
            })
            .count()
            > 0;

        let castletype = match self.modifiers[0] {
            MoveModifier::CastleKingSide => 1,
            MoveModifier::CastleQueenSide => 2,
            _ => 0,
        };

        let is_ep = self.modifiers.contains(&MoveModifier::EnPassant);

        format!(
            "{}{}{}{}{}{}{}",
            fromf, fromr, tof, tor, istake as u8, castletype, is_ep
        )
    }

    pub fn as_tuple(&self) -> (Position, Position, Vec<MoveModifier>, PieceType, Color) {
        (
            self.from,
            self.to,
            self.modifiers.clone(),
            self.piece,
            self.color,
        )
    }
}
