use crate::pieces::piece::Piece;
use crate::types::{color::Color, coordinate::Coordinate, r#move::Move, piece::PieceType};

pub struct King {
    color: Color,
    coords: Coordinate,
}

impl King {
    pub fn new(color: Color, coords: Coordinate) -> King {
        King { coords, color }
    }
}

impl Piece for King {
    fn get_coords(&self) -> &Coordinate {
        &self.coords
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_coords_mut(&mut self) -> &mut Coordinate {
        &mut self.coords
    }

    fn get_type(&self) -> PieceType {
        PieceType::King
    }

    fn get_moves(&self, board: &[Vec<Box<dyn Piece>>]) -> Vec<Move> {
        let mut moves_unchecked = Vec::new();
        let x = self.coords.x;
        let y = self.coords.y;

        moves_unchecked.push(Coordinate::new(x + 1, y));
        moves_unchecked.push(Coordinate::new(x - 1, y));
        moves_unchecked.push(Coordinate::new(x, y + 1));
        moves_unchecked.push(Coordinate::new(x, y - 1));
        moves_unchecked.push(Coordinate::new(x + 1, y + 1));
        moves_unchecked.push(Coordinate::new(x - 1, y - 1));
        moves_unchecked.push(Coordinate::new(x + 1, y - 1));
        moves_unchecked.push(Coordinate::new(x - 1, y + 1));

        let mut moves = moves_unchecked
            .iter()
            .filter(|mv| !mv.is_oob())
            .map(|coord| Move::new(self.coords.copy(), coord.copy(), false))
            .collect::<Vec<Move>>();

        for piece in board.iter().flatten() {
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

        moves
    }
}
