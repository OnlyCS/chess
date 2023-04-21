#![allow(clippy::panic)]

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::{
    core::{color::Color, piece::Piece, piece::PieceType, position::Position, square::Square},
    utils::traits::ToVec,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FileLetter {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
}

impl FileLetter {
    pub fn vec_all() -> Vec<FileLetter> {
        vec![
            FileLetter::A,
            FileLetter::B,
            FileLetter::C,
            FileLetter::D,
            FileLetter::E,
            FileLetter::F,
            FileLetter::G,
            FileLetter::H,
        ]
    }

    pub fn next(&self) -> Result<Self> {
        match self {
            FileLetter::A => Ok(FileLetter::B),
            FileLetter::B => Ok(FileLetter::C),
            FileLetter::C => Ok(FileLetter::D),
            FileLetter::D => Ok(FileLetter::E),
            FileLetter::E => Ok(FileLetter::F),
            FileLetter::F => Ok(FileLetter::G),
            FileLetter::G => Ok(FileLetter::H),
            FileLetter::H => bail!("Cannot go forward from H"),
        }
    }

    pub fn next_loop(&self) -> Self {
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

    pub fn prev(&self) -> Result<Self> {
        match self {
            FileLetter::A => bail!("Cannot go backward from A"),
            FileLetter::B => Ok(FileLetter::A),
            FileLetter::C => Ok(FileLetter::B),
            FileLetter::D => Ok(FileLetter::C),
            FileLetter::E => Ok(FileLetter::D),
            FileLetter::F => Ok(FileLetter::E),
            FileLetter::G => Ok(FileLetter::F),
            FileLetter::H => Ok(FileLetter::G),
        }
    }

    pub fn prev_loop(&self) -> Self {
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

    pub fn fen(&self) -> char {
        <FileLetter as Into<char>>::into(*self)
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
            _ => panic!("FileLetter cannot be created from a character other than a-h"),
        }
    }
}

impl From<FileLetter> for u8 {
    fn from(val: FileLetter) -> Self {
        match val {
            FileLetter::A => 1,
            FileLetter::B => 2,
            FileLetter::C => 3,
            FileLetter::D => 4,
            FileLetter::E => 5,
            FileLetter::F => 6,
            FileLetter::G => 7,
            FileLetter::H => 8,
        }
    }
}

impl From<u8> for FileLetter {
    fn from(val: u8) -> Self {
        match val {
            1 => FileLetter::A,
            2 => FileLetter::B,
            3 => FileLetter::C,
            4 => FileLetter::D,
            5 => FileLetter::E,
            6 => FileLetter::F,
            7 => FileLetter::G,
            8 => FileLetter::H,
            _ => panic!("FileLetter cannot be created from a number other than 1-8"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub letter: FileLetter,
    pub squares: Vec<Square>,
}

impl File {
    pub fn new(letter: FileLetter) -> Self {
        let mut squares: Vec<Square> = Vec::new();

        for rank in 1..9 {
            let position = Position::new(letter, rank);
            squares.push(Square::new(position));
        }

        match letter {
            FileLetter::A | FileLetter::H => {
                squares[0].set_piece(Piece::new(
                    Color::White,
                    Position::new(letter, 1),
                    PieceType::Rook,
                ));
                squares[1].set_piece(Piece::new(
                    Color::White,
                    Position::new(letter, 2),
                    PieceType::Pawn,
                ));

                squares[7].set_piece(Piece::new(
                    Color::Black,
                    Position::new(letter, 8),
                    PieceType::Rook,
                ));
                squares[6].set_piece(Piece::new(
                    Color::Black,
                    Position::new(letter, 7),
                    PieceType::Pawn,
                ));
            }
            FileLetter::B | FileLetter::G => {
                squares[0].set_piece(Piece::new(
                    Color::White,
                    Position::new(letter, 1),
                    PieceType::Knight,
                ));

                squares[7].set_piece(Piece::new(
                    Color::Black,
                    Position::new(letter, 8),
                    PieceType::Knight,
                ));
            }
            FileLetter::C | FileLetter::F => {
                squares[0].set_piece(Piece::new(
                    Color::White,
                    Position::new(letter, 1),
                    PieceType::Bishop,
                ));

                squares[7].set_piece(Piece::new(
                    Color::Black,
                    Position::new(letter, 8),
                    PieceType::Bishop,
                ));
            }
            FileLetter::D => {
                squares[0].set_piece(Piece::new(
                    Color::White,
                    Position::new(letter, 1),
                    PieceType::Queen,
                ));

                squares[7].set_piece(Piece::new(
                    Color::Black,
                    Position::new(letter, 8),
                    PieceType::Queen,
                ));
            }
            FileLetter::E => {
                squares[0].set_piece(Piece::new(
                    Color::White,
                    Position::new(letter, 1),
                    PieceType::King,
                ));

                squares[7].set_piece(Piece::new(
                    Color::Black,
                    Position::new(letter, 8),
                    PieceType::King,
                ))
            }
        }

        squares[1].set_piece(Piece::new(
            Color::White,
            Position::new(letter, 2),
            PieceType::Pawn,
        ));

        squares[6].set_piece(Piece::new(
            Color::Black,
            Position::new(letter, 7),
            PieceType::Pawn,
        ));

        Self { letter, squares }
    }

    pub fn rank(&self, rank: u8) -> Option<&Square> {
        self.squares.get(rank as usize - 1)
    }

    pub fn rank_mut(&mut self, rank: u8) -> Option<&mut Square> {
        self.squares.get_mut(rank as usize - 1)
    }

    pub fn get_squares(&self) -> &Vec<Square> {
        &self.squares
    }

    pub fn get_squares_mut(&mut self) -> &mut Vec<Square> {
        &mut self.squares
    }
}

impl ToVec<Square> for File {
    fn to_vec(&self) -> Vec<&Square> {
        self.squares.iter().collect()
    }

    fn to_vec_mut(&mut self) -> Vec<&mut Square> {
        self.squares.iter_mut().collect()
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new(FileLetter::A)
    }
}
