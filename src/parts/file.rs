use crate::{
    pieces::{bishop::Bishop, king::King, knight::Knight, pawn::Pawn, queen::Queen, rook::Rook},
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
                squares[0].set_piece(Box::new(Rook::new(
                    Color::White,
                    Position::new(letter.clone(), 1),
                )));

                squares[7].set_piece(Box::new(Rook::new(
                    Color::Black,
                    Position::new(letter.clone(), 8),
                )));
                squares[6].set_piece(Box::new(Rook::new(
                    Color::Black,
                    Position::new(letter.clone(), 7),
                )));
            }
            FileLetter::B | FileLetter::G => {
                squares[0].set_piece(Box::new(Knight::new(
                    Color::White,
                    Position::new(letter.clone(), 1),
                )));

                squares[7].set_piece(Box::new(Knight::new(
                    Color::Black,
                    Position::new(letter.clone(), 8),
                )));
                squares[6].set_piece(Box::new(Pawn::new(
                    Color::Black,
                    Position::new(letter.clone(), 7),
                )));
            }
            FileLetter::C | FileLetter::F => {
                squares[0].set_piece(Box::new(Bishop::new(
                    Color::White,
                    Position::new(letter.clone(), 1),
                )));

                squares[7].set_piece(Box::new(Bishop::new(
                    Color::Black,
                    Position::new(letter.clone(), 8),
                )));
                squares[6].set_piece(Box::new(Pawn::new(
                    Color::Black,
                    Position::new(letter.clone(), 7),
                )));
            }
            FileLetter::D => {
                squares[0].set_piece(Box::new(Queen::new(
                    Color::White,
                    Position::new(letter.clone(), 1),
                )));

                squares[7].set_piece(Box::new(Queen::new(
                    Color::Black,
                    Position::new(letter.clone(), 8),
                )));
                squares[6].set_piece(Box::new(Pawn::new(
                    Color::Black,
                    Position::new(letter.clone(), 7),
                )));
            }
            FileLetter::E => {
                squares[0].set_piece(Box::new(King::new(
                    Color::White,
                    Position::new(letter.clone(), 1),
                )));

                squares[7].set_piece(Box::new(King::new(
                    Color::Black,
                    Position::new(letter.clone(), 8),
                )));
            }
        }

        squares[1].set_piece(Box::new(Pawn::new(
            Color::White,
            Position::new(letter.clone(), 2),
        )));

        squares[6].set_piece(Box::new(Pawn::new(
            Color::Black,
            Position::new(letter.clone(), 7),
        )));

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

impl Clone for File {
    fn clone(&self) -> Self {
        Self {
            letter: self.letter.clone(),
            squares: self.squares.to_vec(),
        }
    }
}
