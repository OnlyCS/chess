use crate::{
    types::{color::Color, coordinate::Coordinate, piece::PieceType, r#move::Move},
    utils::{array2d::Array2D, unicode::unicode_from_hex},
};
use std::{error::Error, fmt::Display};

pub trait Piece {
    fn get_coords(&self) -> &Coordinate;
    fn get_color(&self) -> &Color;
    fn get_type(&self) -> PieceType;
    fn get_moves(&self, board: &Array2D<Box<dyn Piece>>) -> Option<Vec<Move>>;
    fn get_coords_mut(&mut self) -> &mut Coordinate;

    fn is_empty(&self) -> bool {
        self.get_type() == PieceType::Empty
    }

    fn move_to(&mut self, to: Coordinate) -> Result<(), Box<dyn Error>> {
        self.get_coords_mut().set(to.x, to.y)
    }

    fn get_value(&self) -> i32 {
        self.get_type().get_value()
    }

    fn to_string(&self) -> String {
        match *self.get_color() {
            Color::White => match self.get_type() {
                PieceType::Pawn => unicode_from_hex("2659"),
                PieceType::Rook => unicode_from_hex("2656"),
                PieceType::Knight => unicode_from_hex("2658"),
                PieceType::Bishop => unicode_from_hex("2657"),
                PieceType::Queen => unicode_from_hex("2655"),
                PieceType::King => unicode_from_hex("2654"),
                PieceType::Empty => Ok(" ".to_string().chars().next().expect("Unknown Error")),
            },
            Color::Black => match self.get_type() {
                PieceType::Pawn => unicode_from_hex("265F"),
                PieceType::Rook => unicode_from_hex("265C"),
                PieceType::Knight => unicode_from_hex("265E"),
                PieceType::Bishop => unicode_from_hex("265D"),
                PieceType::Queen => unicode_from_hex("265B"),
                PieceType::King => unicode_from_hex("265A"),
                PieceType::Empty => Ok(" ".to_string().chars().next().expect("Unknown Error")),
            },
        }
        .expect("Unknown Error")
        .to_string()
    }
}

impl Display for dyn Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Empty {
    color: Color,
    coords: Coordinate,
}

impl Empty {
    pub fn new() -> Empty {
        Empty {
            color: Color::White,
            coords: Coordinate::new(0, 0),
        }
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

impl Piece for Empty {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_coords(&self) -> &Coordinate {
        &self.coords
    }

    fn get_coords_mut(&mut self) -> &mut Coordinate {
        &mut self.coords
    }

    fn get_type(&self) -> PieceType {
        PieceType::Empty
    }

    fn get_moves(&self, _board: &Array2D<Box<dyn Piece>>) -> Option<Vec<Move>> {
        Some(Vec::new())
    }

    fn move_to(&mut self, _to: Coordinate) -> Result<(), Box<dyn Error>> {
        Err("Cannot move empty piece".into())
    }
}
