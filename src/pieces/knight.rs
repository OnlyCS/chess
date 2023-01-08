use crate::pieces::piece::Piece;
use crate::types::{color::Color, coordinate::Coordinate, piece::PieceType, r#move::Move};
use crate::utils::array2d::Array2D;

pub struct Knight {
    color: Color,
    coords: Coordinate,
}

impl Knight {
    pub fn new(color: Color, coords: Coordinate) -> Self {
        Knight { color, coords }
    }
}

impl Piece for Knight {
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
        PieceType::Knight
    }

    fn get_moves(&self, board: &Array2D<Box<dyn Piece>>) -> Option<Vec<Move>> {
        let mut moves_unchecked = Vec::new();
        let x = self.coords.x;
        let y = self.coords.y;

        moves_unchecked.push(Coordinate::new(x + 1, y + 2));
        moves_unchecked.push(Coordinate::new(x + 1, y - 2));
        moves_unchecked.push(Coordinate::new(x - 1, y + 2));
        moves_unchecked.push(Coordinate::new(x - 1, y - 2));
        moves_unchecked.push(Coordinate::new(x + 2, y + 1));
        moves_unchecked.push(Coordinate::new(x + 2, y - 1));
        moves_unchecked.push(Coordinate::new(x - 2, y + 1));
        moves_unchecked.push(Coordinate::new(x - 2, y - 1));

        let mut moves = moves_unchecked
            .iter()
            .filter(|coord| !coord.is_oob())
            .map(|coord| Move::new(self.coords.copy(), coord.copy(), false))
            .collect::<Vec<Move>>();

        for piece in board.flat_iter() {
            let piece_coords = piece.get_coords().copy();

            if piece.get_color() == self.get_color() {
                moves.retain(|mv| mv.to != piece_coords);
            } else {
                for mv in moves.iter_mut() {
                    if mv.to == piece_coords {
                        mv.is_take = true;
                    }
                }
            }
        }

        Some(moves)
    }
}
