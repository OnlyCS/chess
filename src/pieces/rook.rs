use crate::pieces::piece::Piece;
use crate::types::{color::Color, coordinate::Coordinate, r#move::Move, piece::PieceType};

pub struct Rook {
    color: Color,
    coords: Coordinate,
}

impl Rook {
    pub fn new(color: Color, coords: Coordinate) -> Self {
        Rook { color, coords }
    }
}

impl Piece for Rook {
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
        let x = self.coords.x;
        let y = self.coords.y;

        for i in (x..8).chain((0..=x).rev()) {
            let coord = Coordinate::new(i, y);

            if let Some(piece) = board
                .iter()
                .flatten()
                .find(|piece| piece.get_coords() == &coord)
            {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.copy(), coord.copy(), true));
                    break;
                }
            } else {
                moves.push(Move::new(self.coords.copy(), coord.copy(), false));
            }
        }

        for i in (y..8).chain((0..=y).rev()) {
            let coord = Coordinate::new(x, i);

            if let Some(piece) = board
                .iter()
                .flatten()
                .find(|piece| piece.get_coords() == &coord)
            {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.copy(), coord.copy(), true));
                    break;
                }
            } else {
                moves.push(Move::new(self.coords.copy(), coord.copy(), false));
            }
        }

        moves
    }
}
