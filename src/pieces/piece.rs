use std::{error::Error, fmt::Display};

use crate::{
    parts::{board::Board, position::Position},
    types::{color::Color, piece_type::PieceType, r#move::Move},
};

pub fn unicode_from_hex(hex: &str) -> Result<char, Box<dyn Error>> {
    let code = u32::from_str_radix(hex, 16)?;
    let chr = std::char::from_u32(code).ok_or("Invalid unicode code")?;
    Ok(chr)
}

#[derive(Clone, Debug, PartialEq)]
pub struct PieceData {
    pub can_en_passant: bool,
    pub can_double_move: bool,
    pub can_castle: bool,
}

pub trait Piece {
    fn get_color(&self) -> &Color;
    fn get_position(&self) -> &Position;
    fn get_type(&self) -> PieceType;
    fn get_moves(&self, board: &Board) -> Vec<Move>;
    fn to_string(&self) -> String {
        match *self.get_color() {
            Color::White => match self.get_type() {
                PieceType::Pawn => unicode_from_hex("2659"),
                PieceType::Rook => unicode_from_hex("2656"),
                PieceType::Knight => unicode_from_hex("2658"),
                PieceType::Bishop => unicode_from_hex("2657"),
                PieceType::Queen => unicode_from_hex("2655"),
                PieceType::King => unicode_from_hex("2654"),
            },
            Color::Black => match self.get_type() {
                PieceType::Pawn => unicode_from_hex("265F"),
                PieceType::Rook => unicode_from_hex("265C"),
                PieceType::Knight => unicode_from_hex("265E"),
                PieceType::Bishop => unicode_from_hex("265D"),
                PieceType::Queen => unicode_from_hex("265B"),
                PieceType::King => unicode_from_hex("265A"),
            },
        }
        .expect("Unknown Error")
        .to_string()
    }

    fn get_data(&self) -> Option<&PieceData> {
        None
    }
    fn copy(&self) -> Box<dyn Piece + Sync + Send>;
    fn set_position(&mut self, position: Position);
    fn set_data(&mut self, _data: PieceData) {}
}

impl Display for dyn Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Clone for Box<dyn Piece + Sync + Send> {
    fn clone(&self) -> Self {
        self.copy()
    }
}
