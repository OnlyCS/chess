use std::error::Error;

use crate::game::score_counter::ScoreCounter;
use crate::pieces::{
    bishop::Bishop,
    king::King,
    knight::Knight,
    pawn::Pawn,
    piece::{Empty, Piece},
    queen::Queen,
    rook::Rook,
};
use crate::types::{color::Color, coordinate::Coordinate, r#move::Move};
use crate::utils::array2d::Array2D;
use crate::utils::ensure::ensure;
use crate::utils::safe_unwrap::safe_unwrap_option;

pub struct GameManager {
    pub board: Array2D<Box<dyn Piece>>,
    pub turn: Color,
    pub score: ScoreCounter, // lower = black favor, higher = white favor
}

impl GameManager {
    pub fn new() -> GameManager {
        let mut pieces = Vec::new();

        for y in 0..8 {
            for x in 0..8 {
                let piece: Box<dyn Piece> = match y {
                    0 => match x {
                        0 | 7 => Box::new(Rook::new(Color::Black, Coordinate::new(x, y))),
                        1 | 6 => Box::new(Knight::new(Color::Black, Coordinate::new(x, y))),
                        2 | 5 => Box::new(Bishop::new(Color::Black, Coordinate::new(x, y))),
                        3 => Box::new(Queen::new(Color::Black, Coordinate::new(x, y))),
                        4 => Box::new(King::new(Color::Black, Coordinate::new(x, y))),
                        _ => unreachable!(),
                    },
                    1 => Box::new(Pawn::new(Color::Black, Coordinate::new(x, y))),
                    6 => Box::new(Pawn::new(Color::White, Coordinate::new(x, y))),
                    7 => match x {
                        0 | 7 => Box::new(Rook::new(Color::White, Coordinate::new(x, y))),
                        1 | 6 => Box::new(Knight::new(Color::White, Coordinate::new(x, y))),
                        2 | 5 => Box::new(Bishop::new(Color::White, Coordinate::new(x, y))),
                        3 => Box::new(Queen::new(Color::White, Coordinate::new(x, y))),
                        4 => Box::new(King::new(Color::White, Coordinate::new(x, y))),
                        _ => unreachable!(),
                    },
                    _ => Box::new(Empty::new()),
                };

                pieces.push(piece);
            }
        }

        let array = Array2D::new(8, 8, pieces);

        GameManager {
            board: array,
            turn: Color::White,
            score: ScoreCounter::new(),
        }
    }

    pub fn get_piece(&self, coords: Coordinate) -> Option<&dyn Piece> {
        match self.board.get(coords.x, coords.y) {
            Some(piece) => Some(&**piece),
            None => None,
        }
    }

    pub fn get_piece_mut(&mut self, coords: Coordinate) -> Option<&mut dyn Piece> {
        match self.board.get_mut(coords.x, coords.y) {
            Some(piece) => Some(&mut **piece),
            None => None,
        }
    }

    pub fn get_moves_for(&self, color: Color) -> Option<Vec<Move>> {
        let mut moves = Vec::new();

        for x in 0..8 {
            for y in 0..8 {
                let piece = match self.get_piece(Coordinate::new(x, y)) {
                    Some(piece) => piece,
                    None => return None,
                };

                if !piece.is_empty() && piece.get_color() == &color {
                    moves.append(&mut safe_unwrap_option!(piece.get_moves(&self.board)));
                }
            }
        }

        Some(moves)
    }

    pub fn get_moves(&self) -> Option<Vec<Move>> {
        self.get_moves_for(self.turn)
    }

    pub fn move_piece(&mut self, mv: Move) -> Result<(), Box<dyn Error>> {
        let piece = safe_unwrap_option!(self.get_piece(mv.from.copy()), "Unknown Error");
        let target = safe_unwrap_option!(self.get_piece(mv.to.copy()), "Unknown Error");
        let moves = safe_unwrap_option!(piece.get_moves(&self.board), "Unknown Error");
        let mut score = 0;

        ensure!(
            (target.is_empty() && !mv.is_take) || (!target.is_empty() && mv.is_take),
            "Move take status and space status at dest do not match"
        );
        ensure!(
            !mv.from.is_oob() && !mv.to.is_oob(),
            "Move is out of bounds"
        );
        ensure!(
            !piece.is_empty() && *piece.get_color() == self.turn,
            "Not your turn"
        );
        ensure!(moves.contains(&mv), "Move is not valid");

        if !target.is_empty() {
            score += target.get_value();
        }

        let res = safe_unwrap_option!(self.get_piece_mut(mv.from.copy()), "Unknown Error")
            .move_to(mv.to.copy());

        if res.is_ok() {
            match self.turn {
                Color::White => {
                    self.score.add(score);
                }
                Color::Black => {
                    self.score.sub(score);
                }
            };

            self.turn.flip();
        }

        res
    }
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}
