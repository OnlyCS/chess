use crate::pieces::*;
use crate::types::*;

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

    fn get_moves(&self, board: &[Vec<Box<dyn Piece>>]) -> Vec<Move> {
        let mut moves = Vec::new();
        let x = self.coords.x;
        let y = self.coords.y;

        for (i, j) in (1..8).zip(1..8) {
            let coord = Coordinate::new(x + i, y + j);

            if coord.is_oob() {
                break;
            }

            if let Some(piece) = board
                .iter()
                .flatten()
                .find(|piece| piece.get_coords() == &coord)
            {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.clone(), coord.clone(), true));
                    break;
                }
            }
        }

        for (i, j) in (1..8).zip(1..8) {
            let coord = Coordinate::new(x - i, y + j);

            if coord.is_oob() {
                break;
            }

            if let Some(piece) = board
                .iter()
                .flatten()
                .find(|piece| piece.get_coords() == &coord)
            {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.clone(), coord.clone(), true));
                    break;
                }
            }
        }

        for (i, j) in (1..8).zip(1..8) {
            let coord = Coordinate::new(x + i, y - j);

            if coord.is_oob() {
                break;
            }

            if let Some(piece) = board
                .iter()
                .flatten()
                .find(|piece| piece.get_coords() == &coord)
            {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.clone(), coord.clone(), true));
                    break;
                }
            }
        }

        for (i, j) in (1..8).zip(1..8) {
            let coord = Coordinate::new(x - i, y - j);

            if coord.is_oob() {
                break;
            }

            if let Some(piece) = board
                .iter()
                .flatten()
                .find(|piece| piece.get_coords() == &coord)
            {
                if piece.get_color() == self.get_color() {
                    break;
                } else {
                    moves.push(Move::new(self.coords.clone(), coord.clone(), true));
                    break;
                }
            }
        }

        moves
    }
}
