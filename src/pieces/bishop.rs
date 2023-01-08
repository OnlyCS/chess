use crate::pieces::piece::Piece;
use crate::types::{color::Color, coordinate::Coordinate, piece::PieceType, r#move::Move};
use crate::utils::array2d::Array2D;

pub struct Bishop {
    color: Color,
    coords: Coordinate,
}

impl Bishop {
    pub fn new(color: Color, coords: Coordinate) -> Bishop {
        Bishop { coords, color }
    }
}

impl Piece for Bishop {
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
        PieceType::Bishop
    }

    fn get_moves(&self, board: &Array2D<Box<dyn Piece>>) -> Option<Vec<Move>> {
        let mut moves = Vec::new();
        let x = self.coords.x;
        let y = self.coords.y;

        for (i, j) in (1..8).zip(1..8) {
            let coord = Coordinate::new(x + i, y + j);

            if coord.is_oob() {
                break;
            }

            if let Some(piece) = board.flat_iter().find(|piece| piece.get_coords() == &coord) {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.copy(), coord.copy(), true));
                    break;
                }
            }
        }

        for (i, j) in (1..8).zip(1..8) {
            let coord = Coordinate::new(x - i, y + j);

            if coord.is_oob() {
                break;
            }

            if let Some(piece) = board.flat_iter().find(|piece| piece.get_coords() == &coord) {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.copy(), coord.copy(), true));
                    break;
                }
            }
        }

        for (i, j) in (1..8).zip(1..8) {
            let coord = Coordinate::new(x + i, y - j);

            if coord.is_oob() {
                break;
            }

            if let Some(piece) = board.flat_iter().find(|piece| piece.get_coords() == &coord) {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.copy(), coord.copy(), true));
                    break;
                }
            }
        }

        for (i, j) in (1..8).zip(1..8) {
            let coord = Coordinate::new(x - i, y - j);

            if coord.is_oob() {
                break;
            }

            if let Some(piece) = board.flat_iter().find(|piece| piece.get_coords() == &coord) {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.copy(), coord.copy(), true));
                    break;
                }
            }
        }

        Some(moves)
    }
}
