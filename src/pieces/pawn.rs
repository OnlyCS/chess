use std::error::Error;

use crate::pieces::piece::Piece;
use crate::types::{color::Color, coordinate::Coordinate, r#move::Move, piece::PieceType};

pub struct Pawn {
    pub color: Color,
    pub coords: Coordinate,
    pub has_moved: bool,
}

impl Pawn {
    pub fn new(color: Color, coords: Coordinate) -> Pawn {
        Pawn {
            color,
            coords,
            has_moved: false,
        }
    }
}

impl Piece for Pawn {
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
        PieceType::Pawn
    }

    fn get_moves(&self, board: &[Vec<Box<dyn Piece>>]) -> Vec<Move> {
        let mut moves = Vec::new();

        let x = self.coords.x;
        let y = self.coords.y;

        match self.color {
            Color::White => {
                if board[x as usize][y as usize + 1].is_empty() {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x, y + 1),
                        false,
                    ));

                    if board[x as usize][y as usize + 2].is_empty() && !self.has_moved {
                        moves.push(Move::new(
                            self.coords.copy(),
                            Coordinate::new(x, y + 2),
                            false,
                        ));
                    }
                }

                if x > 0 && !board[x as usize - 1][y as usize + 1].is_empty() {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x - 1, y + 1),
                        true,
                    ));
                }

                if x < 7 && !board[x as usize + 1][y as usize + 1].is_empty() {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x + 1, y + 1),
                        true,
                    ));
                }
            }
            Color::Black => {
                if board[x as usize][y as usize - 1].is_empty() {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x, y - 1),
                        false,
                    ));

                    if board[x as usize][y as usize - 2].is_empty() && !self.has_moved {
                        moves.push(Move::new(
                            self.coords.copy(),
                            Coordinate::new(x, y - 2),
                            false,
                        ));
                    }
                }

                if x > 0 && !board[x as usize - 1][y as usize - 1].is_empty() {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x - 1, y - 1),
                        true,
                    ));
                }

                if x < 7 && !board[x as usize + 1][y as usize - 1].is_empty() {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x + 1, y - 1),
                        true,
                    ));
                }
            }
        }

        Vec::new()
    }

    fn move_to(&mut self, to: Coordinate) -> Result<(), Box<dyn Error>> {
        let set = self.coords.set(to.x, to.y);

        if set.is_ok() {
            self.has_moved = true;
        }

        set
    }
}
