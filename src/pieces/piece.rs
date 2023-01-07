use std::error::Error;
use crate::types::{color::Color, coordinate::Coordinate, r#move::Move, piece::PieceType};

pub trait Piece {
    fn get_coords(&self) -> &Coordinate;
    fn get_color(&self) -> &Color;
    fn get_type(&self) -> PieceType;
    fn get_moves(&self, board: &[Vec<Box<dyn Piece>>]) -> Vec<Move>;
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

    fn get_moves(&self, _board: &[Vec<Box<dyn Piece>>]) -> Vec<Move> {
        Vec::new()
    }

    fn move_to(&mut self, _to: Coordinate) -> Result<(), Box<dyn Error>> {
        Err("Cannot move empty piece".into())
    }
}
