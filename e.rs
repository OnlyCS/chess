#![feature(prelude_import)]
#![warn(clippy::unwrap_used, clippy::panic)]
#![allow(clippy::needless_update, clippy::borrowed_box)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use crate::ui::root::Root;
use intuitive::terminal::Terminal;
pub mod game {
    pub mod chess {
        use crate::{parts::board::Board, types::{color::Color, r#move::Move}};
        pub struct Chess {
            board: Board,
            turn: Color,
        }
        impl Chess {
            pub fn new(board: Board) -> Self {
                Self { board, turn: Color::White }
            }
            pub fn get_board(&self) -> &Board {
                &self.board
            }
            pub fn get_moves(&self) -> Vec<Move> {
                self.board.get_moves_for(self.turn)
            }
        }
        impl Default for Chess {
            fn default() -> Self {
                Self::new(Board::new())
            }
        }
        impl Clone for Chess {
            fn clone(&self) -> Self {
                Self {
                    board: self.board.clone(),
                    turn: self.turn,
                }
            }
        }
    }
}
pub mod parts {
    pub mod board {
        use std::error::Error;
        use crate::types::{
            color::Color, etc::ToResult, file_letter::FileLetter, r#move::Move,
        };
        use super::{file::File, position::Position, square::Square};
        pub struct Board {
            files: Vec<File>,
        }
        impl Board {
            pub fn new() -> Board {
                let mut board = Self { files: Vec::new() };
                for letter in FileLetter::vec_all() {
                    board.files.push(File::new(letter));
                }
                board
            }
            pub fn file<T: Into<FileLetter> + Clone>(
                &self,
                letter: &T,
            ) -> Option<&File> {
                self.files.iter().find(|file| file.letter == letter.clone().into())
            }
            pub fn file_mut<T: Into<FileLetter> + Clone>(
                &mut self,
                letter: &T,
            ) -> Option<&mut File> {
                self.files.iter_mut().find(|file| file.letter == letter.clone().into())
            }
            pub fn square(&self, position: &Position) -> Option<&Square> {
                self.file(&position.file)?.rank(position.rank)
            }
            pub fn square_mut(&mut self, position: &Position) -> Option<&mut Square> {
                self.file_mut(&position.file)?.rank_mut(position.rank)
            }
            pub fn make_move(&mut self, mv: &Move) -> Result<(), Box<dyn Error>> {
                let mut piece_from = self
                    .square(&mv.from)
                    .to_result("Move \"from\" invalid".into())?
                    .get_piece()
                    .to_result("No piece at \"from\"".into())?
                    .clone();
                let moves = piece_from.get_moves(self);
                if !moves.contains(mv) {
                    return Err("Invalid move".into());
                }
                let square_to = self
                    .square_mut(&mv.to)
                    .to_result("Move \"to\" invalid".into())?;
                piece_from.set_position(mv.to.clone());
                square_to.set_piece(piece_from);
                self.square_mut(&mv.from)
                    .to_result("Move \"from\" invalid".into())?
                    .clear();
                Ok(())
            }
            pub fn get_moves_for(&self, color: Color) -> Vec<Move> {
                let mut moves = Vec::new();
                for square in self.clone() {
                    if let Some(piece) = square.get_piece() {
                        if *piece.get_color() == color {
                            moves.append(&mut piece.get_moves(self));
                        }
                    }
                }
                moves
            }
            pub fn get_files(&self) -> &Vec<File> {
                &self.files
            }
        }
        impl Default for Board {
            fn default() -> Self {
                Self::new()
            }
        }
        impl IntoIterator for Board {
            type Item = Square;
            type IntoIter = std::vec::IntoIter<Self::Item>;
            fn into_iter(self) -> Self::IntoIter {
                self.files
                    .into_iter()
                    .flat_map(|file| file.into_iter())
                    .collect::<Vec<_>>()
                    .into_iter()
            }
        }
        impl Clone for Board {
            fn clone(&self) -> Self {
                Self { files: self.files.to_vec() }
            }
        }
    }
    pub mod file {
        use crate::{
            pieces::{
                bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen,
                rook::Rook,
            },
            types::{color::Color, file_letter::FileLetter},
        };
        use super::{position::Position, square::Square};
        pub struct File {
            pub letter: FileLetter,
            squares: Vec<Square>,
        }
        impl File {
            pub fn new(letter: FileLetter) -> Self {
                let mut squares = Vec::new();
                for rank in 1..9 {
                    let position = Position::new(letter.clone(), rank);
                    squares.push(Square::new(position));
                }
                match letter {
                    FileLetter::A | FileLetter::H => {
                        squares[0]
                            .set_piece(
                                Box::new(
                                    Rook::new(Color::White, Position::new(letter.clone(), 1)),
                                ),
                            );
                        squares[7]
                            .set_piece(
                                Box::new(
                                    Rook::new(Color::Black, Position::new(letter.clone(), 8)),
                                ),
                            );
                        squares[6]
                            .set_piece(
                                Box::new(
                                    Rook::new(Color::Black, Position::new(letter.clone(), 7)),
                                ),
                            );
                    }
                    FileLetter::B | FileLetter::G => {
                        squares[0]
                            .set_piece(
                                Box::new(
                                    Knight::new(Color::White, Position::new(letter.clone(), 1)),
                                ),
                            );
                        squares[7]
                            .set_piece(
                                Box::new(
                                    Knight::new(Color::Black, Position::new(letter.clone(), 8)),
                                ),
                            );
                        squares[6]
                            .set_piece(
                                Box::new(
                                    Pawn::new(Color::Black, Position::new(letter.clone(), 7)),
                                ),
                            );
                    }
                    FileLetter::C | FileLetter::F => {
                        squares[0]
                            .set_piece(
                                Box::new(
                                    Bishop::new(Color::White, Position::new(letter.clone(), 1)),
                                ),
                            );
                        squares[7]
                            .set_piece(
                                Box::new(
                                    Bishop::new(Color::Black, Position::new(letter.clone(), 8)),
                                ),
                            );
                        squares[6]
                            .set_piece(
                                Box::new(
                                    Pawn::new(Color::Black, Position::new(letter.clone(), 7)),
                                ),
                            );
                    }
                    FileLetter::D => {
                        squares[0]
                            .set_piece(
                                Box::new(
                                    Queen::new(Color::White, Position::new(letter.clone(), 1)),
                                ),
                            );
                        squares[7]
                            .set_piece(
                                Box::new(
                                    Queen::new(Color::Black, Position::new(letter.clone(), 8)),
                                ),
                            );
                        squares[6]
                            .set_piece(
                                Box::new(
                                    Pawn::new(Color::Black, Position::new(letter.clone(), 7)),
                                ),
                            );
                    }
                    FileLetter::E => {
                        squares[0]
                            .set_piece(
                                Box::new(
                                    King::new(Color::White, Position::new(letter.clone(), 1)),
                                ),
                            );
                        squares[7]
                            .set_piece(
                                Box::new(
                                    King::new(Color::Black, Position::new(letter.clone(), 8)),
                                ),
                            );
                    }
                }
                squares[1]
                    .set_piece(
                        Box::new(
                            Pawn::new(Color::White, Position::new(letter.clone(), 2)),
                        ),
                    );
                squares[6]
                    .set_piece(
                        Box::new(
                            Pawn::new(Color::Black, Position::new(letter.clone(), 7)),
                        ),
                    );
                Self { letter, squares }
            }
            pub fn rank(&self, rank: u8) -> Option<&Square> {
                self.squares.get(rank as usize - 1)
            }
            pub fn rank_mut(&mut self, rank: u8) -> Option<&mut Square> {
                self.squares.get_mut(rank as usize - 1)
            }
        }
        impl IntoIterator for File {
            type Item = Square;
            type IntoIter = std::vec::IntoIter<Self::Item>;
            fn into_iter(self) -> Self::IntoIter {
                self.squares.into_iter()
            }
        }
        impl Clone for File {
            fn clone(&self) -> Self {
                Self {
                    letter: self.letter.clone(),
                    squares: self.squares.to_vec(),
                }
            }
        }
    }
    pub mod position {
        use crate::types::file_letter::FileLetter;
        pub struct Position {
            pub file: FileLetter,
            pub rank: u8,
        }
        impl Position {
            pub fn new<T: Into<FileLetter>>(file: T, rank: u8) -> Self {
                Self { file: file.into(), rank }
            }
            pub fn copy(&self) -> Self {
                Self::new(self.file.clone(), self.rank)
            }
            pub fn is_oob(&self) -> bool {
                self.file < FileLetter::A || self.file > FileLetter::H || self.rank < 1
                    || self.rank > 8
            }
            pub fn up(&mut self) {
                if self.rank < 8 {
                    self.rank += 1;
                }
            }
            pub fn down(&mut self) {
                if self.rank > 1 {
                    self.rank -= 1;
                }
            }
            pub fn left(&mut self) {
                if self.file > FileLetter::A {
                    self.file -= 1;
                }
            }
            pub fn right(&mut self) {
                if self.file < FileLetter::H {
                    self.file += 1;
                }
            }
        }
        impl PartialEq for Position {
            fn eq(&self, other: &Self) -> bool {
                self.file == other.file && self.rank == other.rank
            }
        }
        impl Default for Position {
            fn default() -> Self {
                Self::new('a', 1)
            }
        }
        impl Clone for Position {
            fn clone(&self) -> Self {
                Self::new(self.file.clone(), self.rank)
            }
        }
    }
    pub mod square {
        use crate::pieces::piece::Piece;
        use super::position::Position;
        pub struct Square {
            position: Position,
            piece: Option<Box<dyn Piece>>,
        }
        impl Square {
            pub fn new(position: Position) -> Self {
                Self { position, piece: None }
            }
            pub fn set_piece(&mut self, piece: Box<dyn Piece>) {
                self.piece = Some(piece);
            }
            pub fn get_piece(&self) -> Option<&Box<dyn Piece>> {
                self.piece.as_ref()
            }
            pub fn get_piece_mut(&mut self) -> Option<&mut Box<dyn Piece>> {
                self.piece.as_mut()
            }
            pub fn get_position(&self) -> &Position {
                &self.position
            }
            pub fn is_empty(&self) -> bool {
                self.piece.is_none()
            }
            pub fn clear(&mut self) {
                self.piece = None;
            }
        }
        impl Clone for Square {
            fn clone(&self) -> Self {
                Self {
                    position: self.position.clone(),
                    piece: self.piece.clone(),
                }
            }
        }
    }
}
pub mod pieces {
    pub mod bishop {
        use crate::{
            parts::{board::Board, position::Position},
            types::{
                color::Color, piece_type::PieceType,
                r#move::{Move, MoveFilter, MoveModifier},
            },
        };
        use super::piece::Piece;
        pub struct Bishop {
            color: Color,
            position: Position,
        }
        impl Bishop {
            pub fn new(color: Color, position: Position) -> Self {
                Self { color, position }
            }
        }
        impl Piece for Bishop {
            fn get_color(&self) -> &Color {
                &self.color
            }
            fn get_position(&self) -> &Position {
                &self.position
            }
            fn get_type(&self) -> PieceType {
                PieceType::Bishop
            }
            fn get_moves(&self, board: &Board) -> Vec<Move> {
                let mut moves = Vec::new();
                for i in 1..=8 {
                    let mut position = self.position.clone();
                    position.file += i;
                    position.rank += i;
                    if let Some(square) = board.square(&position) {
                        if let Some(piece) = square.get_piece() {
                            if piece.get_color() != &self.color {
                                moves
                                    .push(
                                        Move::new(
                                            self.position.clone(),
                                            position,
                                            Some(
                                                <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([MoveModifier::Capture]),
                                                ),
                                            ),
                                        ),
                                    );
                            }
                            break;
                        } else {
                            moves.push(Move::new(self.position.clone(), position, None));
                        }
                    } else {
                        break;
                    }
                }
                for i in 1..=8 {
                    let mut position = self.position.clone();
                    position.file -= i;
                    position.rank += i;
                    if let Some(square) = board.square(&position) {
                        if let Some(piece) = square.get_piece() {
                            if piece.get_color() != &self.color {
                                moves
                                    .push(
                                        Move::new(
                                            self.position.clone(),
                                            position,
                                            Some(
                                                <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([MoveModifier::Capture]),
                                                ),
                                            ),
                                        ),
                                    );
                            }
                            break;
                        } else {
                            moves.push(Move::new(self.position.clone(), position, None));
                        }
                    } else {
                        break;
                    }
                }
                for i in 1..=8 {
                    let mut position = self.position.clone();
                    position.file += i;
                    position.rank -= i;
                    if let Some(square) = board.square(&position) {
                        if let Some(piece) = square.get_piece() {
                            if piece.get_color() != &self.color {
                                moves
                                    .push(
                                        Move::new(
                                            self.position.clone(),
                                            position,
                                            Some(
                                                <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([MoveModifier::Capture]),
                                                ),
                                            ),
                                        ),
                                    );
                            }
                            break;
                        } else {
                            moves.push(Move::new(self.position.clone(), position, None));
                        }
                    } else {
                        break;
                    }
                }
                for i in 1..=8 {
                    let mut position = self.position.clone();
                    position.file -= i;
                    position.rank -= i;
                    if let Some(square) = board.square(&position) {
                        if let Some(piece) = square.get_piece() {
                            if piece.get_color() != &self.color {
                                moves
                                    .push(
                                        Move::new(
                                            self.position.clone(),
                                            position,
                                            Some(
                                                <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([MoveModifier::Capture]),
                                                ),
                                            ),
                                        ),
                                    );
                            }
                            break;
                        } else {
                            moves.push(Move::new(self.position.clone(), position, None));
                        }
                    } else {
                        break;
                    }
                }
                moves.retain(|mv| !mv.to.is_oob() && !mv.from.is_oob());
                moves.filter_king_check(board, self.color);
                moves
            }
            fn copy(&self) -> Box<dyn Piece> {
                Box::new(Self {
                    color: self.color,
                    position: self.position.clone(),
                })
            }
            fn set_position(&mut self, position: Position) {
                self.position = position;
            }
        }
    }
    pub mod king {
        use crate::{
            parts::{board::Board, position::Position},
            types::{
                color::Color, file_letter::FileLetter, piece_type::PieceType,
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
                let mut moves = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file + 1,
                                self.position.rank + 1,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file + 1,
                                self.position.rank,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file + 1,
                                self.position.rank - 1,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file,
                                self.position.rank + 1,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file,
                                self.position.rank - 1,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file - 1,
                                self.position.rank + 1,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file - 1,
                                self.position.rank,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file - 1,
                                self.position.rank - 1,
                            ),
                            None,
                        ),
                    ]),
                );
                let mut remove = Vec::new();
                for (i, m) in moves.iter_mut().enumerate() {
                    if let Some(square) = board.square(&m.to) {
                        if let Some(piece) = square.get_piece() {
                            if *piece.get_color() == self.color {
                                remove.push(i)
                            } else {
                                m.modifiers
                                    .as_mut()
                                    .unwrap_or(&mut Vec::new())
                                    .push(MoveModifier::Capture);
                            }
                        }
                    } else {
                        remove.push(i)
                    }
                }
                for i in remove.iter().rev() {
                    moves.remove(*i);
                }
                moves.retain(|m| !m.from.is_oob() && !m.to.is_oob());
                moves
            }
            fn copy(&self) -> Box<dyn Piece> {
                Box::new(Self {
                    color: self.color,
                    position: self.position.clone(),
                    data: self.data.clone(),
                })
            }
            fn set_position(&mut self, position: Position) {
                self.position = position;
            }
        }
    }
    pub mod knight {
        use crate::{
            parts::{board::Board, position::Position},
            types::{color::Color, piece_type::PieceType, r#move::{Move, MoveModifier}},
        };
        use super::piece::Piece;
        pub struct Knight {
            color: Color,
            position: Position,
        }
        impl Knight {
            pub fn new(color: Color, position: Position) -> Self {
                Self { color, position }
            }
        }
        impl Piece for Knight {
            fn get_color(&self) -> &Color {
                &self.color
            }
            fn get_position(&self) -> &Position {
                &self.position
            }
            fn get_type(&self) -> PieceType {
                PieceType::Knight
            }
            fn get_moves(&self, board: &Board) -> Vec<Move> {
                let mut moves = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file + 1,
                                self.position.rank + 2,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file + 1,
                                self.position.rank - 2,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file - 1,
                                self.position.rank + 2,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file - 1,
                                self.position.rank - 2,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file + 2,
                                self.position.rank + 1,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file + 2,
                                self.position.rank - 1,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file - 2,
                                self.position.rank + 1,
                            ),
                            None,
                        ),
                        Move::new(
                            self.position.clone(),
                            Position::new(
                                self.position.clone().file - 2,
                                self.position.rank - 1,
                            ),
                            None,
                        ),
                    ]),
                );
                let mut remove = Vec::new();
                for (i, m) in moves.iter_mut().enumerate() {
                    if let Some(square) = board.square(&m.to) {
                        if let Some(piece) = square.get_piece() {
                            if piece.get_color() == &self.color {
                                remove.push(i);
                            } else {
                                m
                                    .modifiers = Some(
                                    <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([MoveModifier::Capture]),
                                    ),
                                );
                            }
                        }
                    } else {
                        remove.push(i);
                    }
                }
                for i in remove.iter().rev() {
                    moves.remove(*i);
                }
                moves
            }
            fn copy(&self) -> Box<dyn Piece> {
                Box::new(Self {
                    color: self.color,
                    position: self.position.clone(),
                })
            }
            fn set_position(&mut self, position: Position) {
                self.position = position;
            }
        }
    }
    pub mod pawn {
        use crate::{
            parts::{board::Board, position::Position},
            types::{color::Color, piece_type::PieceType, r#move::{Move, MoveModifier}},
        };
        use super::piece::{Piece, PieceData};
        pub struct Pawn {
            color: Color,
            position: Position,
            pub data: PieceData,
        }
        impl Pawn {
            pub fn new(color: Color, position: Position) -> Self {
                Self {
                    data: PieceData {
                        can_en_passant: false,
                        can_double_move: match color {
                            Color::White => position.rank == 2,
                            Color::Black => position.rank == 7,
                        },
                        can_castle: false,
                    },
                    color,
                    position,
                }
            }
        }
        impl Piece for Pawn {
            fn get_color(&self) -> &Color {
                &self.color
            }
            fn get_position(&self) -> &Position {
                &self.position
            }
            fn get_type(&self) -> PieceType {
                PieceType::Pawn
            }
            fn get_moves(&self, board: &Board) -> Vec<Move> {
                let mut moves = Vec::new();
                if self.data.can_double_move {
                    if let Some(square)
                        = board
                            .square(
                                &Position::new(
                                    self.position.clone().file,
                                    self.position.rank + 2,
                                ),
                            )
                    {
                        if square.is_empty() {
                            moves
                                .push(
                                    Move::new(
                                        self.position.clone(),
                                        Position::new(
                                            self.position.clone().file,
                                            self.position.rank + 2,
                                        ),
                                        None,
                                    ),
                                );
                        }
                    }
                }
                if let Some(square)
                    = board
                        .square(
                            &Position::new(
                                self.position.clone().file,
                                self.position.rank + 1,
                            ),
                        )
                {
                    if square.is_empty() {
                        moves
                            .push(
                                Move::new(
                                    self.position.clone(),
                                    Position::new(
                                        self.position.clone().file,
                                        self.position.rank + 1,
                                    ),
                                    None,
                                ),
                            );
                    }
                }
                if let Some(square)
                    = board
                        .square(
                            &Position::new(
                                self.position.clone().file + 1,
                                self.position.rank + 1,
                            ),
                        )
                {
                    if let Some(piece) = square.get_piece() {
                        if piece.get_color() != self.get_color() {
                            moves
                                .push(
                                    Move::new(
                                        self.position.clone(),
                                        Position::new(
                                            self.position.clone().file + 1,
                                            self.position.rank + 1,
                                        ),
                                        None,
                                    ),
                                );
                        }
                    }
                }
                if let Some(square)
                    = board
                        .square(
                            &Position::new(
                                self.position.clone().file - 1,
                                self.position.rank + 1,
                            ),
                        )
                {
                    if let Some(piece) = square.get_piece() {
                        if piece.get_color() != self.get_color() {
                            moves
                                .push(
                                    Move::new(
                                        self.position.clone(),
                                        Position::new(
                                            self.position.clone().file - 1,
                                            self.position.rank + 1,
                                        ),
                                        None,
                                    ),
                                );
                        }
                    }
                }
                if let Some(ep_square)
                    = board
                        .square(
                            &Position::new(
                                self.position.clone().file + 1,
                                self.position.rank,
                            ),
                        )
                {
                    if let Some(ep_piece) = ep_square.get_piece() {
                        if ep_piece.get_color() != self.get_color()
                            && ep_piece.get_type() == PieceType::Pawn
                            && ep_piece.get_data().expect("unreachable").can_en_passant
                        {
                            moves
                                .push(
                                    Move::new(
                                        self.position.clone(),
                                        Position::new(
                                            self.position.clone().file + 1,
                                            self.position.rank + 1,
                                        ),
                                        None,
                                    ),
                                );
                        }
                    }
                }
                for m in moves.iter_mut() {
                    let mut modifiers = Vec::new();
                    if let Some(mods) = m.clone().modifiers {
                        modifiers.extend(mods);
                    }
                    if m.to.rank == 8 && self.color == Color::White
                        || m.to.rank == 1 && self.color == Color::Black
                    {
                        modifiers.push(MoveModifier::Promotion);
                    }
                    m.modifiers = Some(modifiers);
                }
                moves
            }
            fn copy(&self) -> Box<dyn Piece> {
                Box::new(Self {
                    color: self.color,
                    position: self.position.clone(),
                    data: self.data.clone(),
                })
            }
            fn set_position(&mut self, position: Position) {
                self.position = position;
            }
        }
    }
    pub mod piece {
        use std::{error::Error, fmt::Display};
        use crate::{
            parts::{board::Board, position::Position},
            types::{color::Color, piece_type::PieceType, r#move::Move},
        };
        pub fn unicode_from_hex(hex: &str) -> Result<char, Box<dyn Error>> {
            let code = u32::from_str_radix(hex, 16)?;
            let chr = std::char::from_u32(code).ok_or("Invalid unicode code")?;
            Ok(chr)
        }
        pub struct PieceData {
            pub can_en_passant: bool,
            pub can_double_move: bool,
            pub can_castle: bool,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PieceData {
            #[inline]
            fn clone(&self) -> PieceData {
                PieceData {
                    can_en_passant: ::core::clone::Clone::clone(&self.can_en_passant),
                    can_double_move: ::core::clone::Clone::clone(&self.can_double_move),
                    can_castle: ::core::clone::Clone::clone(&self.can_castle),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PieceData {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "PieceData",
                    "can_en_passant",
                    &self.can_en_passant,
                    "can_double_move",
                    &self.can_double_move,
                    "can_castle",
                    &&self.can_castle,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PieceData {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PieceData {
            #[inline]
            fn eq(&self, other: &PieceData) -> bool {
                self.can_en_passant == other.can_en_passant
                    && self.can_double_move == other.can_double_move
                    && self.can_castle == other.can_castle
            }
        }
        pub trait Piece {
            fn get_color(&self) -> &Color;
            fn get_position(&self) -> &Position;
            fn get_type(&self) -> PieceType;
            fn get_moves(&self, board: &Board) -> Vec<Move>;
            fn to_string(&self) -> String {
                match *self.get_color() {
                    Color::White => {
                        match self.get_type() {
                            PieceType::Pawn => unicode_from_hex("2659"),
                            PieceType::Rook => unicode_from_hex("2656"),
                            PieceType::Knight => unicode_from_hex("2658"),
                            PieceType::Bishop => unicode_from_hex("2657"),
                            PieceType::Queen => unicode_from_hex("2655"),
                            PieceType::King => unicode_from_hex("2654"),
                        }
                    }
                    Color::Black => {
                        match self.get_type() {
                            PieceType::Pawn => unicode_from_hex("265F"),
                            PieceType::Rook => unicode_from_hex("265C"),
                            PieceType::Knight => unicode_from_hex("265E"),
                            PieceType::Bishop => unicode_from_hex("265D"),
                            PieceType::Queen => unicode_from_hex("265B"),
                            PieceType::King => unicode_from_hex("265A"),
                        }
                    }
                }
                    .expect("Unknown Error")
                    .to_string()
            }
            fn get_data(&self) -> Option<&PieceData> {
                None
            }
            fn copy(&self) -> Box<dyn Piece>;
            fn set_position(&mut self, position: Position);
        }
        impl Display for dyn Piece {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{0}", self.to_string()))
            }
        }
        impl Clone for Box<dyn Piece> {
            fn clone(&self) -> Self {
                self.copy()
            }
        }
    }
    pub mod queen {
        use crate::{
            parts::{board::Board, position::Position},
            types::{color::Color, piece_type::PieceType, r#move::Move},
        };
        use super::{bishop::Bishop, piece::Piece, rook::Rook};
        pub struct Queen {
            color: Color,
            position: Position,
        }
        impl Queen {
            pub fn new(color: Color, position: Position) -> Self {
                Self { color, position }
            }
        }
        impl Piece for Queen {
            fn get_color(&self) -> &Color {
                &self.color
            }
            fn get_position(&self) -> &Position {
                &self.position
            }
            fn get_type(&self) -> PieceType {
                PieceType::Queen
            }
            fn get_moves(&self, board: &Board) -> Vec<Move> {
                let mut moves = Vec::new();
                moves
                    .extend(
                        Rook::new(self.color, self.position.clone()).get_moves(board),
                    );
                moves
                    .extend(
                        Bishop::new(self.color, self.position.clone()).get_moves(board),
                    );
                moves
            }
            fn copy(&self) -> Box<dyn Piece> {
                Box::new(Self {
                    color: self.color,
                    position: self.position.clone(),
                })
            }
            fn set_position(&mut self, position: Position) {
                self.position = position;
            }
        }
    }
    pub mod rook {
        use crate::{
            parts::{board::Board, position::Position},
            types::{
                color::Color, file_letter::FileLetter, piece_type::PieceType,
                r#move::{Move, MoveModifier},
            },
        };
        use super::piece::{Piece, PieceData};
        pub struct Rook {
            color: Color,
            position: Position,
            data: PieceData,
        }
        impl Rook {
            pub fn new(color: Color, position: Position) -> Self {
                Self {
                    data: PieceData {
                        can_en_passant: false,
                        can_double_move: false,
                        can_castle: position.file == FileLetter::A
                            || position.file == FileLetter::H,
                    },
                    color,
                    position,
                }
            }
        }
        impl Piece for Rook {
            fn get_color(&self) -> &Color {
                &self.color
            }
            fn get_position(&self) -> &Position {
                &self.position
            }
            fn get_type(&self) -> PieceType {
                PieceType::Rook
            }
            fn get_moves(&self, board: &Board) -> Vec<Move> {
                let mut moves = Vec::new();
                for i in 1..8 {
                    let mut position = self.position.clone();
                    position.rank += i;
                    if let Some(square) = board.square(&position) {
                        if let Some(piece) = square.get_piece() {
                            if *piece.get_color() != self.color {
                                moves
                                    .push(
                                        Move::new(
                                            self.position.clone(),
                                            position,
                                            Some(
                                                <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([MoveModifier::Capture]),
                                                ),
                                            ),
                                        ),
                                    );
                            }
                            break;
                        } else {
                            moves.push(Move::new(self.position.clone(), position, None));
                        }
                    } else {
                        break;
                    }
                }
                for i in 1..8 {
                    let mut position = self.position.clone();
                    position.file += i;
                    if let Some(square) = board.square(&position) {
                        if let Some(piece) = square.get_piece() {
                            if *piece.get_color() != self.color {
                                moves
                                    .push(
                                        Move::new(
                                            self.position.clone(),
                                            position,
                                            Some(
                                                <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([MoveModifier::Capture]),
                                                ),
                                            ),
                                        ),
                                    );
                            }
                            break;
                        } else {
                            moves.push(Move::new(self.position.clone(), position, None));
                        }
                    } else {
                        break;
                    }
                }
                for i in 1..8 {
                    let mut position = self.position.clone();
                    position.rank -= i;
                    if let Some(square) = board.square(&position) {
                        if let Some(piece) = square.get_piece() {
                            if *piece.get_color() != self.color {
                                moves
                                    .push(
                                        Move::new(
                                            self.position.clone(),
                                            position,
                                            Some(
                                                <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([MoveModifier::Capture]),
                                                ),
                                            ),
                                        ),
                                    );
                            }
                            break;
                        } else {
                            moves.push(Move::new(self.position.clone(), position, None));
                        }
                    } else {
                        break;
                    }
                }
                for i in 1..8 {
                    let mut position = self.position.clone();
                    position.file -= i;
                    if let Some(square) = board.square(&position) {
                        if let Some(piece) = square.get_piece() {
                            if *piece.get_color() != self.color {
                                moves
                                    .push(
                                        Move::new(
                                            self.position.clone(),
                                            position,
                                            Some(
                                                <[_]>::into_vec(
                                                    #[rustc_box]
                                                    ::alloc::boxed::Box::new([MoveModifier::Capture]),
                                                ),
                                            ),
                                        ),
                                    );
                            }
                            break;
                        } else {
                            moves.push(Move::new(self.position.clone(), position, None));
                        }
                    } else {
                        break;
                    }
                }
                moves.retain(|m| !m.to.is_oob() && !m.from.is_oob());
                moves
            }
            fn copy(&self) -> Box<dyn Piece> {
                Box::new(Self {
                    color: self.color,
                    position: self.position.clone(),
                    data: self.data.clone(),
                })
            }
            fn set_position(&mut self, position: Position) {
                self.position = position;
            }
        }
    }
}
pub mod types {
    pub mod color {
        pub enum Color {
            Black,
            White,
        }
        impl Color {
            pub fn get_opposite(&self) -> Color {
                match self {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                }
            }
            pub fn get_string(&self) -> String {
                match self {
                    Color::White => "White".to_string(),
                    Color::Black => "Black".to_string(),
                }
            }
            pub fn get_char(&self) -> char {
                match self {
                    Color::White => 'w',
                    Color::Black => 'b',
                }
            }
            pub fn flip(&mut self) {
                *self = self.other();
            }
            pub fn other(&self) -> Color {
                match self {
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                }
            }
        }
        impl TryFrom<i32> for Color {
            type Error = Box<dyn std::error::Error>;
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    0 => Ok(Color::White),
                    1 => Ok(Color::Black),
                    _ => Err("Invalid Color".into()),
                }
            }
        }
        impl From<bool> for Color {
            fn from(value: bool) -> Self {
                match value {
                    true => Color::White,
                    false => Color::Black,
                }
            }
        }
        impl PartialEq for Color {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Color::White, Color::White) | (Color::Black, Color::Black) => true,
                    _ => false,
                }
            }
        }
        impl Clone for Color {
            fn clone(&self) -> Self {
                match self {
                    Color::White => Color::White,
                    Color::Black => Color::Black,
                }
            }
        }
        impl Copy for Color {}
    }
    pub mod etc {
        use std::error::Error;
        pub trait ToResult<T> {
            fn to_result(self, error: Box<dyn Error>) -> Result<T, Box<dyn Error>>;
        }
        impl<T> ToResult<T> for Option<T> {
            fn to_result(self, error: Box<dyn Error>) -> Result<T, Box<dyn Error>> {
                match self {
                    Some(t) => Ok(t),
                    None => Err(error),
                }
            }
        }
    }
    pub mod file_letter {
        #![allow(clippy::panic)]
        use std::ops::{Add, AddAssign, Sub, SubAssign};
        pub enum FileLetter {
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
        }
        impl FileLetter {
            pub fn vec_all() -> Vec<FileLetter> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        FileLetter::A,
                        FileLetter::B,
                        FileLetter::C,
                        FileLetter::D,
                        FileLetter::E,
                        FileLetter::F,
                        FileLetter::G,
                        FileLetter::H,
                    ]),
                )
            }
            pub fn inc(&self) -> Self {
                match self {
                    FileLetter::A => FileLetter::B,
                    FileLetter::B => FileLetter::C,
                    FileLetter::C => FileLetter::D,
                    FileLetter::D => FileLetter::E,
                    FileLetter::E => FileLetter::F,
                    FileLetter::F => FileLetter::G,
                    FileLetter::G => FileLetter::H,
                    FileLetter::H => FileLetter::A,
                }
            }
            pub fn dec(&self) -> Self {
                match self {
                    FileLetter::A => FileLetter::H,
                    FileLetter::B => FileLetter::A,
                    FileLetter::C => FileLetter::B,
                    FileLetter::D => FileLetter::C,
                    FileLetter::E => FileLetter::D,
                    FileLetter::F => FileLetter::E,
                    FileLetter::G => FileLetter::F,
                    FileLetter::H => FileLetter::G,
                }
            }
        }
        impl From<FileLetter> for char {
            fn from(file: FileLetter) -> Self {
                match file {
                    FileLetter::A => 'a',
                    FileLetter::B => 'b',
                    FileLetter::C => 'c',
                    FileLetter::D => 'd',
                    FileLetter::E => 'e',
                    FileLetter::F => 'f',
                    FileLetter::G => 'g',
                    FileLetter::H => 'h',
                }
            }
        }
        impl From<char> for FileLetter {
            fn from(letter: char) -> Self {
                match letter {
                    'a' => FileLetter::A,
                    'b' => FileLetter::B,
                    'c' => FileLetter::C,
                    'd' => FileLetter::D,
                    'e' => FileLetter::E,
                    'f' => FileLetter::F,
                    'g' => FileLetter::G,
                    'h' => FileLetter::H,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "FileLetter cannot be created from a character other than a-h"
                            ),
                        )
                    }
                }
            }
        }
        impl From<FileLetter> for u8 {
            fn from(val: FileLetter) -> Self {
                match val {
                    FileLetter::A => 0,
                    FileLetter::B => 1,
                    FileLetter::C => 2,
                    FileLetter::D => 3,
                    FileLetter::E => 4,
                    FileLetter::F => 5,
                    FileLetter::G => 6,
                    FileLetter::H => 7,
                }
            }
        }
        impl PartialEq for FileLetter {
            fn eq(&self, other: &Self) -> bool {
                let self_num: u8 = Into::into(self.clone());
                let other_num: u8 = Into::into(other.clone());
                self_num == other_num
            }
        }
        impl PartialOrd for FileLetter {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                let self_num: u8 = Into::into(self.clone());
                let other_num: u8 = Into::into(other.clone());
                self_num.partial_cmp(&other_num)
            }
        }
        impl Clone for FileLetter {
            fn clone(&self) -> Self {
                match self {
                    FileLetter::A => FileLetter::A,
                    FileLetter::B => FileLetter::B,
                    FileLetter::C => FileLetter::C,
                    FileLetter::D => FileLetter::D,
                    FileLetter::E => FileLetter::E,
                    FileLetter::F => FileLetter::F,
                    FileLetter::G => FileLetter::G,
                    FileLetter::H => FileLetter::H,
                }
            }
        }
        impl Add<u8> for FileLetter {
            type Output = Self;
            fn add(self, rhs: u8) -> Self::Output {
                let mut new_file = self;
                for _ in 0..rhs {
                    new_file = new_file.inc();
                }
                new_file
            }
        }
        impl AddAssign<u8> for FileLetter {
            fn add_assign(&mut self, rhs: u8) {
                *self = self.clone() + rhs;
            }
        }
        impl Sub<u8> for FileLetter {
            type Output = Self;
            fn sub(self, rhs: u8) -> Self::Output {
                let mut new_file = self;
                for _ in 0..rhs {
                    new_file = new_file.dec();
                }
                new_file
            }
        }
        impl SubAssign<u8> for FileLetter {
            fn sub_assign(&mut self, rhs: u8) {
                *self = self.clone() - rhs;
            }
        }
    }
    pub mod r#move {
        use crate::parts::{board::Board, position::Position, square::Square};
        use super::{color::Color, piece_type::PieceType};
        pub enum MoveModifier {
            Capture,
            EnPassant,
            Promotion,
            Castle,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MoveModifier {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MoveModifier {
            #[inline]
            fn eq(&self, other: &MoveModifier) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MoveModifier {
            #[inline]
            fn clone(&self) -> MoveModifier {
                match self {
                    MoveModifier::Capture => MoveModifier::Capture,
                    MoveModifier::EnPassant => MoveModifier::EnPassant,
                    MoveModifier::Promotion => MoveModifier::Promotion,
                    MoveModifier::Castle => MoveModifier::Castle,
                }
            }
        }
        pub struct Move {
            pub from: Position,
            pub to: Position,
            pub modifiers: Option<Vec<MoveModifier>>,
        }
        impl Move {
            pub fn new(
                from: Position,
                to: Position,
                modifiers: Option<Vec<MoveModifier>>,
            ) -> Move {
                Move { from, to, modifiers }
            }
        }
        impl Clone for Move {
            fn clone(&self) -> Self {
                Move {
                    from: self.from.clone(),
                    to: self.to.clone(),
                    modifiers: self.modifiers.clone(),
                }
            }
        }
        impl PartialEq for Move {
            fn eq(&self, other: &Self) -> bool {
                self.from == other.from && self.to == other.to
                    && self.modifiers == other.modifiers
            }
        }
        impl Default for Move {
            fn default() -> Self {
                Self::new(Position::default(), Position::default(), None)
            }
        }
        pub trait MoveFilter {
            fn filter_king_check(&mut self, board: &Board, color: Color);
        }
        impl MoveFilter for Vec<Move> {
            fn filter_king_check(&mut self, board: &Board, color: Color) {
                self.retain(|_| {
                    let mut king_in_check = false;
                    for othermv in board.get_moves_for(color.other()) {
                        let mut this_board = board.clone();
                        this_board
                            .make_move(&othermv)
                            .expect("Failed to check for king check");
                        let is_king = this_board
                            .into_iter()
                            .collect::<Vec<Square>>()
                            .iter()
                            .any(|sq| {
                                if let Some(piece) = sq.get_piece() {
                                    piece.get_type() == PieceType::King
                                        && *piece.get_color() == color
                                } else {
                                    false
                                }
                            });
                        if !is_king {
                            king_in_check = true;
                            break;
                        }
                    }
                    !king_in_check
                });
            }
        }
    }
    pub mod piece_type {
        use std::{cmp::Ordering, fmt::{Display, Formatter}};
        pub enum PieceType {
            Pawn,
            Knight,
            Bishop,
            Rook,
            Queen,
            King,
        }
        impl PieceType {
            pub fn get_value(&self) -> i32 {
                match self {
                    PieceType::Pawn => 1,
                    PieceType::Knight => 3,
                    PieceType::Bishop => 3,
                    PieceType::Rook => 5,
                    PieceType::Queen => 9,
                    PieceType::King => 100,
                }
            }
        }
        impl PartialOrd for PieceType {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.get_value().cmp(&other.get_value()))
            }
        }
        impl PartialEq for PieceType {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (PieceType::Pawn, PieceType::Pawn)
                    | (PieceType::Knight, PieceType::Knight)
                    | (PieceType::Bishop, PieceType::Bishop)
                    | (PieceType::Rook, PieceType::Rook)
                    | (PieceType::Queen, PieceType::Queen)
                    | (PieceType::King, PieceType::King) => true,
                    _ => false,
                }
            }
        }
        impl Display for PieceType {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    PieceType::Pawn => f.write_fmt(format_args!("P")),
                    PieceType::Knight => f.write_fmt(format_args!("Kn")),
                    PieceType::Bishop => f.write_fmt(format_args!("B")),
                    PieceType::Rook => f.write_fmt(format_args!("R")),
                    PieceType::Queen => f.write_fmt(format_args!("Q")),
                    PieceType::King => f.write_fmt(format_args!("K")),
                }
            }
        }
    }
}
pub mod ui {
    pub mod board {
        use intuitive::{
            components::*, event::{KeyHandler, MouseHandler},
            *,
        };
        use super::{data::UIFileData, file::File};
        pub struct Board {
            pub on_key: KeyHandler,
            pub on_mouse: MouseHandler,
            pub board_data: Vec<UIFileData>,
        }
        #[automatically_derived]
        impl ::core::default::Default for Board {
            #[inline]
            fn default() -> Board {
                Board {
                    on_key: ::core::default::Default::default(),
                    on_mouse: ::core::default::Default::default(),
                    board_data: ::core::default::Default::default(),
                }
            }
        }
        impl Board {
            pub fn new(
                on_key: KeyHandler,
                on_mouse: MouseHandler,
                board_data: Vec<UIFileData>,
            ) -> ::intuitive::components::Any {
                Self {
                    on_key,
                    on_mouse,
                    board_data,
                }
                    .into()
            }
        }
        impl ::intuitive::components::Component for Board {
            fn render(&self) -> ::intuitive::element::Any {
                let Board { on_key, on_mouse, board_data } = self;
                {
                    HStack {
                        on_key: on_key
                            .try_into()
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "try into failed for argument: \'{0}\'", "on_key"
                                        ),
                                    );
                                    res
                                },
                            ),
                        on_mouse: on_mouse
                            .try_into()
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "try into failed for argument: \'{0}\'", "on_mouse"
                                        ),
                                    );
                                    res
                                },
                            ),
                        children: [
                            File {
                                data: board_data[0]
                                    .clone()
                                    .pieces
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "data"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: board_data[0]
                                    .clone()
                                    .select
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            File {
                                data: board_data[1]
                                    .clone()
                                    .pieces
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "data"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: board_data[1]
                                    .clone()
                                    .select
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            File {
                                data: board_data[2]
                                    .clone()
                                    .pieces
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "data"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: board_data[2]
                                    .clone()
                                    .select
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            File {
                                data: board_data[3]
                                    .clone()
                                    .pieces
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "data"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: board_data[3]
                                    .clone()
                                    .select
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            File {
                                data: board_data[4]
                                    .clone()
                                    .pieces
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "data"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: board_data[4]
                                    .clone()
                                    .select
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            File {
                                data: board_data[5]
                                    .clone()
                                    .pieces
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "data"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: board_data[5]
                                    .clone()
                                    .select
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            File {
                                data: board_data[6]
                                    .clone()
                                    .pieces
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "data"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: board_data[6]
                                    .clone()
                                    .select
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            File {
                                data: board_data[7]
                                    .clone()
                                    .pieces
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "data"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: board_data[7]
                                    .clone()
                                    .select
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                        ]
                            .into(),
                        ..Default::default()
                    }
                        .into()
                }
            }
        }
    }
    pub mod data {
        use super::selection::SelectionType;
        pub struct UIFileData {
            pub pieces: Vec<String>,
            pub select: Vec<Option<SelectionType>>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UIFileData {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UIFileData",
                    "pieces",
                    &self.pieces,
                    "select",
                    &&self.select,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UIFileData {
            #[inline]
            fn clone(&self) -> UIFileData {
                UIFileData {
                    pieces: ::core::clone::Clone::clone(&self.pieces),
                    select: ::core::clone::Clone::clone(&self.select),
                }
            }
        }
        pub struct SelectData {
            pub selection: Option<SelectionType>,
            pub piece: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SelectData {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SelectData",
                    "selection",
                    &self.selection,
                    "piece",
                    &&self.piece,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SelectData {
            #[inline]
            fn clone(&self) -> SelectData {
                SelectData {
                    selection: ::core::clone::Clone::clone(&self.selection),
                    piece: ::core::clone::Clone::clone(&self.piece),
                }
            }
        }
        impl UIFileData {
            pub fn create_from(data: Vec<SelectData>) -> Self {
                let mut pieces = Vec::new();
                let mut selections = Vec::new();
                for d in data {
                    pieces.push(d.piece);
                    selections.push(d.selection);
                }
                Self { pieces, select: selections }
            }
        }
    }
    pub mod file {
        use super::{selection::SelectionType, square::Square};
        use intuitive::{components::VStack, *};
        pub struct File {
            pub data: Vec<String>,
            pub selected: Vec<Option<SelectionType>>,
        }
        #[automatically_derived]
        impl ::core::default::Default for File {
            #[inline]
            fn default() -> File {
                File {
                    data: ::core::default::Default::default(),
                    selected: ::core::default::Default::default(),
                }
            }
        }
        impl File {
            pub fn new(
                data: Vec<String>,
                selected: Vec<Option<SelectionType>>,
            ) -> ::intuitive::components::Any {
                Self { data, selected }.into()
            }
        }
        impl ::intuitive::components::Component for File {
            fn render(&self) -> ::intuitive::element::Any {
                let File { data, selected } = self;
                {
                    VStack {
                        flex: [1, 1, 1, 1, 1, 1, 1, 1]
                            .try_into()
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "try into failed for argument: \'{0}\'", "flex"
                                        ),
                                    );
                                    res
                                },
                            ),
                        children: [
                            Square {
                                piece: data[0]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "piece"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: selected[0]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            Square {
                                piece: data[1]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "piece"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: selected[1]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            Square {
                                piece: data[2]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "piece"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: selected[2]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            Square {
                                piece: data[3]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "piece"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: selected[3]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            Square {
                                piece: data[4]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "piece"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: selected[4]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            Square {
                                piece: data[5]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "piece"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: selected[5]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            Square {
                                piece: data[6]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "piece"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: selected[6]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                            Square {
                                piece: data[7]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "piece"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                selected: selected[7]
                                    .clone()
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "selected"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                ..Default::default()
                            }
                                .into(),
                        ]
                            .into(),
                        ..Default::default()
                    }
                        .into()
                }
            }
        }
    }
    pub mod root {
        use crossterm::event::MouseEvent;
        use intuitive::{event::handler::Propagate, state::use_state, *};
        use crate::{game::chess::Chess, parts::{position::Position, square::Square}};
        use super::{
            board::Board, data::{SelectData, UIFileData},
            selection::{Selection, SelectionMode},
        };
        pub struct Root {}
        #[automatically_derived]
        impl ::core::default::Default for Root {
            #[inline]
            fn default() -> Root {
                Root {}
            }
        }
        impl Root {
            pub fn new() -> ::intuitive::components::Any {
                Self {}.into()
            }
        }
        impl ::intuitive::components::Component for Root {
            fn render(&self) -> ::intuitive::element::Any {
                let Root {} = self;
                {
                    let game = Chess::default();
                    let select_mode = use_state(|| SelectionMode::SelectPiece);
                    let selection = use_state(|| Selection {
                        hover: Position::default(),
                        selected: None,
                        avaliable: ::alloc::vec::Vec::new(),
                    });
                    let board_data = game
                        .get_board()
                        .get_files()
                        .iter()
                        .map(|f| f.clone().into_iter().collect::<Vec<Square>>())
                        .map(|p| {
                            p.iter()
                                .map(|s| SelectData {
                                    selection: selection.get().has(s.get_position()),
                                    piece: s
                                        .get_piece()
                                        .map(|p| p.to_string())
                                        .unwrap_or(" ".to_string()),
                                })
                                .rev()
                                .collect::<Vec<SelectData>>()
                        })
                        .collect::<Vec<Vec<SelectData>>>()
                        .iter()
                        .map(|f| UIFileData::create_from(f.to_vec()))
                        .collect::<Vec<UIFileData>>();
                    let key_hander = {
                        use super::selection::SelectionMode;
                        move |event| {
                            match select_mode.get() {
                                SelectionMode::SelectPiece => {
                                    use intuitive::event::{
                                        self, KeyCode::{self, *},
                                        KeyEvent,
                                    };
                                    match event {
                                        KeyEvent { code: Char('q'), .. } => event::quit(),
                                        KeyEvent { code: Char('w'), .. } => {
                                            selection.mutate(|s| s.hover.up())
                                        }
                                        KeyEvent { code: Char('a'), .. } => {
                                            selection.mutate(|s| s.hover.left())
                                        }
                                        KeyEvent { code: Char('s'), .. } => {
                                            selection.mutate(|s| s.hover.down())
                                        }
                                        KeyEvent { code: Char('d'), .. } => {
                                            selection.mutate(|s| s.hover.right())
                                        }
                                        KeyEvent { code: KeyCode::Enter, .. } => {
                                            select_mode.set(SelectionMode::SelectMove);
                                        }
                                        _ => {}
                                    }
                                }
                                SelectionMode::SelectMove => {
                                    use intuitive::event::{self, KeyCode::*, KeyEvent};
                                    match event {
                                        KeyEvent { code: Char('q'), .. } => event::quit(),
                                        KeyEvent { code: Char('w'), .. } => {
                                            selection.mutate(|s| s.hover.up())
                                        }
                                        KeyEvent { code: Char('a'), .. } => {
                                            selection.mutate(|s| s.hover.left())
                                        }
                                        KeyEvent { code: Char('s'), .. } => {
                                            selection.mutate(|s| s.hover.down())
                                        }
                                        KeyEvent { code: Char('d'), .. } => {
                                            selection.mutate(|s| s.hover.right())
                                        }
                                        KeyEvent { code: Esc, .. } => {
                                            select_mode.set(SelectionMode::SelectPiece);
                                            selection
                                                .mutate(|s| {
                                                    s.selected = None;
                                                    s.avaliable = ::alloc::vec::Vec::new();
                                                });
                                        }
                                        _ => {}
                                    }
                                }
                            };
                            Propagate::Next
                        }
                    };
                    let mouse_handler = { move |_: MouseEvent| Propagate::Stop };
                    Board {
                        on_key: key_hander
                            .try_into()
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "try into failed for argument: \'{0}\'", "on_key"
                                        ),
                                    );
                                    res
                                },
                            ),
                        on_mouse: mouse_handler
                            .try_into()
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "try into failed for argument: \'{0}\'", "on_mouse"
                                        ),
                                    );
                                    res
                                },
                            ),
                        board_data: board_data
                            .try_into()
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "try into failed for argument: \'{0}\'", "board_data"
                                        ),
                                    );
                                    res
                                },
                            ),
                        ..Default::default()
                    }
                        .into()
                }
            }
        }
    }
    pub mod selection {
        use crate::parts::position::Position;
        pub enum SelectionType {
            Hover,
            Selected,
            Available,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SelectionType {
            #[inline]
            fn clone(&self) -> SelectionType {
                match self {
                    SelectionType::Hover => SelectionType::Hover,
                    SelectionType::Selected => SelectionType::Selected,
                    SelectionType::Available => SelectionType::Available,
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SelectionType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        SelectionType::Hover => "Hover",
                        SelectionType::Selected => "Selected",
                        SelectionType::Available => "Available",
                    },
                )
            }
        }
        pub enum SelectionMode {
            SelectPiece,
            SelectMove,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for SelectionMode {
            #[inline]
            fn clone(&self) -> SelectionMode {
                match self {
                    SelectionMode::SelectPiece => SelectionMode::SelectPiece,
                    SelectionMode::SelectMove => SelectionMode::SelectMove,
                }
            }
        }
        impl Default for SelectionType {
            fn default() -> Self {
                Self::Hover
            }
        }
        pub struct Selection {
            pub hover: Position,
            pub selected: Option<Position>,
            pub avaliable: Vec<Position>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Selection {
            #[inline]
            fn clone(&self) -> Selection {
                Selection {
                    hover: ::core::clone::Clone::clone(&self.hover),
                    selected: ::core::clone::Clone::clone(&self.selected),
                    avaliable: ::core::clone::Clone::clone(&self.avaliable),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Selection {
            #[inline]
            fn default() -> Selection {
                Selection {
                    hover: ::core::default::Default::default(),
                    selected: ::core::default::Default::default(),
                    avaliable: ::core::default::Default::default(),
                }
            }
        }
        impl Selection {
            pub fn has(&self, pos: &Position) -> Option<SelectionType> {
                if self.hover == *pos {
                    Some(SelectionType::Hover)
                } else if let Some(selected) = &self.selected {
                    if selected == pos {
                        Some(SelectionType::Selected)
                    } else if self.avaliable.contains(pos) {
                        Some(SelectionType::Available)
                    } else {
                        None
                    }
                } else if self.avaliable.contains(pos) {
                    Some(SelectionType::Available)
                } else {
                    None
                }
            }
        }
    }
    pub mod square {
        use intuitive::{
            components::*, style::{Color, Modifier, Style},
            *,
        };
        use super::selection::SelectionType;
        pub struct Square {
            pub piece: String,
            pub selected: Option<SelectionType>,
        }
        #[automatically_derived]
        impl ::core::default::Default for Square {
            #[inline]
            fn default() -> Square {
                Square {
                    piece: ::core::default::Default::default(),
                    selected: ::core::default::Default::default(),
                }
            }
        }
        impl Square {
            pub fn new(
                piece: String,
                selected: Option<SelectionType>,
            ) -> ::intuitive::components::Any {
                Self { piece, selected }.into()
            }
        }
        impl ::intuitive::components::Component for Square {
            fn render(&self) -> ::intuitive::element::Any {
                let Square { piece, selected } = self;
                {
                    let style = match selected {
                        Some(SelectionType::Hover) => {
                            Style::new(Some(Color::Blue), None, Modifier::empty())
                        }
                        Some(SelectionType::Selected) => {
                            Style::new(Some(Color::Green), None, Modifier::empty())
                        }
                        Some(SelectionType::Available) => {
                            Style::new(Some(Color::Yellow), None, Modifier::empty())
                        }
                        None => Style::new(None, None, Modifier::empty()),
                    };
                    Section {
                        border: style
                            .try_into()
                            .expect(
                                &{
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "try into failed for argument: \'{0}\'", "border"
                                        ),
                                    );
                                    res
                                },
                            ),
                        children: [
                            HStack {
                                flex: [5, 12, 4]
                                    .try_into()
                                    .expect(
                                        &{
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "try into failed for argument: \'{0}\'", "flex"
                                                ),
                                            );
                                            res
                                        },
                                    ),
                                children: [
                                    Empty { ..Default::default() }.into(),
                                    Text {
                                        text: piece
                                            .try_into()
                                            .expect(
                                                &{
                                                    let res = ::alloc::fmt::format(
                                                        format_args!(
                                                            "try into failed for argument: \'{0}\'", "text"
                                                        ),
                                                    );
                                                    res
                                                },
                                            ),
                                        ..Default::default()
                                    }
                                        .into(),
                                    Empty { ..Default::default() }.into(),
                                ]
                                    .into(),
                                ..Default::default()
                            }
                                .into(),
                        ]
                            .into(),
                        ..Default::default()
                    }
                        .into()
                }
            }
        }
    }
}
fn main() {
    let mut terminal = Terminal::new(Root::new()).expect("Failed to create UI");
    terminal.run().expect("Failed to run UI");
}
