use crate::pieces::{piece::Piece, rook::Rook, bishop::Bishop};
use crate::types::{color::Color, coordinate::Coordinate, r#move::Move, piece::PieceType};

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

    fn get_moves(&self, board: &[Vec<Box<dyn Piece>>]) -> Vec<Move> {
        let mut moves = Vec::new();

        moves.extend(Rook::new(self.color, self.coords.copy()).get_moves(board));
        moves.extend(Bishop::new(self.color, self.coords.copy()).get_moves(board));

        moves
    }
}
