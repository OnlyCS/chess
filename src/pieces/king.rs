use crate::{
    parts::{board::Board, position::Position},
    types::{
        color::Color,
        file_letter::FileLetter,
        piece_type::PieceType,
        r#move::{Move, MoveModifier},
    },
};

use super::piece::{Piece, PieceData};

pub struct King {
    color: Color,
    position: Position,
    data: PieceData,
}

impl King {
    pub fn new(color: Color, position: Position) -> Self {
        Self {
            data: PieceData {
                can_en_passant: false,
                can_double_move: false,
                can_castle: match color {
                    Color::White => position == Position::new(FileLetter::E, 1),
                    Color::Black => position == Position::new(FileLetter::E, 8),
                },
            },
            color,
            position,
        }
    }

    fn incheck_without_castle(&self, board: &Board) -> bool {
        let mut working_board = board.clone();

        for s in working_board.get_squares_mut() {
            if let Some(p) = s.get_piece_mut() {
                if let Some(data) = p.get_data() {
                    let mut data = data.clone();

                    data.can_castle = false;

                    p.set_data(data);
                }
            }
        }

        working_board.is_check(self.color)
    }
}

impl Piece for King {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_type(&self) -> PieceType {
        PieceType::King
    }

    fn get_moves(&self, board: &Board) -> Vec<Move> {
        let move_nums = vec![
            self.position.up(1).map(|i| i.right(1)),
            self.position.up(1).map(|i| i.left(1)),
            self.position.down(1).map(|i| i.right(1)),
            self.position.down(1).map(|i| i.left(1)),
            Ok(self.position.up(1)),
            Ok(self.position.down(1)),
            Ok(self.position.right(1)),
            Ok(self.position.left(1)),
        ];

        let mut moves = Vec::new();

        for position in move_nums.into_iter().flatten().flatten() {
            let mut capture = false;
            let mut keep = true;

            if let Some(Some(piece)) = board.square(&position).map(|x| x.get_piece()) {
                if piece.get_color() != &self.color {
                    capture = true;
                } else {
                    keep = false;
                }
            }

            if keep {
                let mut modifiers = Vec::new();
                if capture {
                    modifiers.push(MoveModifier::Capture);
                }

                moves.push(Move::new(self.position.clone(), position, modifiers));
            }
        }

        if self.data.can_castle && !self.incheck_without_castle(board) {
            // check if king can castle kingside
            if board
                .square(&Position::new(FileLetter::H, self.position.rank))
                .and_then(|x| x.get_piece())
                .filter(|x| x.get_type() == PieceType::Rook)
                .filter(|x| x.get_color() == &self.color)
                .filter(|x| x.get_data().map(|x| x.can_castle).unwrap_or(false))
                .is_some()
            {
                let mut keep = true;

                for i in 1..=2 {
                    if board
                        .square(&self.position.right(i).expect("unreachable"))
                        .and_then(|x| x.get_piece())
                        .is_some()
                    {
                        keep = false;
                        break;
                    }

                    let mut working_board = board.clone();

                    working_board
                        .make_move(&Move::new(
                            self.position.clone(),
                            self.position.right(i).expect("unreachable"),
                            Vec::new(),
                        ))
                        .expect("unreachable");

                    if working_board.is_check(self.color) {
                        keep = false;
                        break;
                    }
                }

                if keep {
                    moves.push(Move::new(
                        self.position.clone(),
                        self.position.right(2).expect("unreachable"),
                        vec![MoveModifier::CastleKingSide],
                    ));
                }
            }

            // check if king can castle queenside
            if board
                .square(&Position::new(FileLetter::A, self.position.rank))
                .and_then(|x| x.get_piece())
                .filter(|x| x.get_type() == PieceType::Rook)
                .filter(|x| x.get_color() == &self.color)
                .filter(|x| x.get_data().map(|x| x.can_castle).unwrap_or(false))
                .is_some()
            {
                let mut keep = true;

                for i in 1..=3 {
                    if board
                        .square(&self.position.left(i).expect("unreachable"))
                        .and_then(|x| x.get_piece())
                        .is_some()
                    {
                        keep = false;
                        break;
                    }

                    let mut working_board = board.clone();

                    working_board
                        .make_move(&Move::new(
                            self.position.clone(),
                            self.position.left(i).expect("unreachable"),
                            Vec::new(),
                        ))
                        .expect("unreachable");

                    if working_board.is_check(self.color) {
                        keep = false;
                        break;
                    }
                }

                if keep {
                    moves.push(Move::new(
                        self.position.clone(),
                        self.position.left(2).expect("unreachable"),
                        vec![MoveModifier::CastleQueenSide],
                    ));
                }
            }
        }

        moves.retain(|m| !m.from.is_oob() && !m.to.is_oob());

        moves
    }

    fn copy(&self) -> Box<dyn Piece + Sync + Send> {
        Box::new(Self {
            color: self.color,
            position: self.position.clone(),
            data: self.data.clone(),
        })
    }

    fn set_position(&mut self, position: Position) {
        if self.data.can_castle {
            self.data.can_castle = false;
        }

        self.position = position;
    }

    fn set_data(&mut self, data: PieceData) {
        self.data = data;
    }

    fn get_data(&self) -> Option<&PieceData> {
        Some(&self.data)
    }
}
