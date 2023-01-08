use crate::pieces::{bishop::Bishop, piece::Piece, rook::Rook};
use crate::types::{color::Color, coordinate::Coordinate, piece::PieceType, r#move::Move};
use crate::utils::array2d::Array2D;
use crate::utils::safe_unwrap::safe_unwrap_option;

pub struct Queen {
    color: Color,
    coords: Coordinate,
}

impl Queen {
    pub fn new(color: Color, coords: Coordinate) -> Queen {
        Queen { color, coords }
    }
}

impl Piece for Queen {
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
        PieceType::Queen
    }

    fn get_moves(&self, board: &Array2D<Box<dyn Piece>>) -> Option<Vec<Move>> {
        let mut moves: Vec<Move> = Vec::new();

        moves.extend(safe_unwrap_option!(Rook::new(
            self.color,
            self.coords.copy()
        )
        .get_moves(board)));
        moves.extend(safe_unwrap_option!(Bishop::new(
            self.color,
            self.coords.copy()
        )
        .get_moves(board)));

        Some(moves)
    }
}
