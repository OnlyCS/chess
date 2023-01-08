use std::error::Error;

use crate::pieces::piece::Piece;
use crate::types::{color::Color, coordinate::Coordinate, piece::PieceType, r#move::Move};
use crate::utils::array2d::Array2D;
use crate::utils::safe_unwrap::safe_unwrap_option;

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

    fn get_moves(&self, board: &Array2D<Box<dyn Piece>>) -> Option<Vec<Move>> {
        let mut moves = Vec::new();

        let x = self.coords.x;
        let y = self.coords.y;

        match self.color {
            Color::White => {
                if !Coordinate::new(x, y + 1).is_oob()
                    && safe_unwrap_option!(board.get(x, y + 1)).is_empty()
                {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x, y + 1),
                        false,
                    ));

                    if !Coordinate::new(x, y + 2).is_oob()
                        && safe_unwrap_option!(board.get(x, y + 2)).is_empty()
                        && !self.has_moved
                    {
                        moves.push(Move::new(
                            self.coords.copy(),
                            Coordinate::new(x, y + 2),
                            false,
                        ));
                    }
                }

                if !Coordinate::new(x - 1, y + 1).is_oob()
                    && !safe_unwrap_option!(board.get(x - 1, y + 1)).is_empty()
                {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x - 1, y + 1),
                        true,
                    ));
                }

                if !Coordinate::new(x + 1, y + 1).is_oob()
                    && !safe_unwrap_option!(board.get(x + 1, y + 1)).is_empty()
                {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x + 1, y + 1),
                        true,
                    ));
                }
            }
            Color::Black => {
                if !Coordinate::new(x, y - 1).is_oob()
                    && safe_unwrap_option!(board.get(x, y - 1)).is_empty()
                {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x, y - 1),
                        false,
                    ));

                    if !Coordinate::new(x, y - 2).is_oob()
                        && safe_unwrap_option!(board.get(x, y - 2)).is_empty()
                        && !self.has_moved
                    {
                        moves.push(Move::new(
                            self.coords.copy(),
                            Coordinate::new(x, y - 2),
                            false,
                        ));
                    }
                }

                if !Coordinate::new(x - 1, y - 1).is_oob()
                    && !safe_unwrap_option!(board.get(x - 1, y - 1)).is_empty()
                {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x - 1, y - 1),
                        true,
                    ));
                }

                if !Coordinate::new(x + 1, y - 1).is_oob()
                    && !safe_unwrap_option!(board.get(x + 1, y - 1)).is_empty()
                {
                    moves.push(Move::new(
                        self.coords.copy(),
                        Coordinate::new(x + 1, y - 1),
                        true,
                    ));
                }
            }
        }

        Some(moves)
    }

    fn move_to(&mut self, to: Coordinate) -> Result<(), Box<dyn Error>> {
        let set = self.coords.set(to.x, to.y);

        if set.is_ok() {
            self.has_moved = true;
        }

        set
    }
}
