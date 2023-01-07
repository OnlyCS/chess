use std::error::Error;

use crate::pieces::{piece::{Piece, Empty}, bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook};
use crate::types::{color::Color, coordinate::Coordinate, r#move::Move, piece::PieceType};
use crate::game::score_counter::ScoreCounter;
use crate::utils::ensure::{ensure, ensure_eq};

pub struct Game {
    pub board: Vec<Vec<Box<dyn Piece>>>,
    pub turn: Color,
    pub score: ScoreCounter, // lower = black favor, higher = white favor
}

impl Game {
    pub fn new() -> Game {
        let mut board: Vec<Vec<Box<dyn Piece>>> = Vec::new();

        for x in 0..8 {
            let mut row: Vec<Box<dyn Piece>> = Vec::new();

            for y in 0..8 {
                match y {
                    1 | 6 => row.push(Box::new(Pawn::new(
                        Color::from(y == 1),
                        Coordinate::new(x, y),
                    ))),

                    0 | 7 => match x {
                        0 | 7 => row.push(Box::new(Rook::new(
                            Color::from(y == 0),
                            Coordinate::new(x, y),
                        ))),
                        1 | 6 => row.push(Box::new(Knight::new(
                            Color::from(y == 0),
                            Coordinate::new(x, y),
                        ))),
                        2 | 5 => row.push(Box::new(Bishop::new(
                            Color::from(y == 0),
                            Coordinate::new(x, y),
                        ))),
                        3 => row.push(Box::new(Queen::new(
                            Color::from(y == 0),
                            Coordinate::new(x, y),
                        ))),
                        4 => row.push(Box::new(King::new(
                            Color::from(y == 0),
                            Coordinate::new(x, y),
                        ))),
                        _ => row.push(Box::new(Empty::new())),
                    },

                    _ => row.push(Box::new(Empty::new())),
                }
            }

            board.push(row);
        }

        Game {
            board,
            turn: Color::White,
            score: ScoreCounter::new(),
        }
    }

    fn mod_score(&mut self, score: i32) {
        self.score.add(score);
    }

    pub fn get_piece(&self, coords: Coordinate) -> &dyn Piece {
        &*self.board[coords.x as usize][coords.y as usize]
    }

    pub fn get_piece_mut(&mut self, coords: Coordinate) -> &mut dyn Piece {
        &mut *self.board[coords.x as usize][coords.y as usize]
    }

    pub fn get_moves_for(&self, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();

        for x in 0..8 {
            for y in 0..8 {
                let piece = self.get_piece(Coordinate::new(x, y));

                if !piece.is_empty() && piece.get_color() == &color {
                    moves.append(&mut piece.get_moves(&self.board));
                }
            }
        }

        moves
    }

    pub fn get_moves(&self) -> Vec<Move> {
        self.get_moves_for(self.turn)
    }

    pub fn move_piece(&mut self, mv: Move) -> Result<(), Box<dyn Error>> {
        let piece = self.get_piece(mv.from.copy());
        let target = self.get_piece(mv.to.copy());
        let moves = piece.get_moves(&self.board);
        let mut score = 0;

        ensure!((target.is_empty() && !mv.is_take) || (!target.is_empty() && mv.is_take), "Move take status and space status at dest do not match");
        ensure!(!mv.from.is_oob() && !mv.to.is_oob(), "Move is out of bounds");
        ensure!(!piece.is_empty() && *piece.get_color() == self.turn, "Not your turn");
        ensure!(moves.contains(&mv), "Move is not valid");

        if !target.is_empty() {
            score += target.get_value();
        }

        let res = self.get_piece_mut(mv.from.copy()).move_to(mv.to.copy());

        if res.is_ok() {
            match self.turn {
                Color::White => {
                    self.score.add(score);
                },
                Color::Black => {
                    self.score.sub(score);
                },
            };

            self.turn.flip();
        }

        res
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}