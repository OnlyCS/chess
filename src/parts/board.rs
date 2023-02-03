use std::error::Error;

use crate::types::{color::Color, etc::ToResult, file_letter::FileLetter, r#move::Move};

use super::{file::File, position::Position, square::Square};

pub struct Board {
    files: Vec<File>,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Self { files: Vec::new() };
        for letter in ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].iter() {
            let file = match File::new(*letter) {
                Ok(f) => f,
                Err(_) => unreachable!(),
            };

            board.files.push(file);
        }

        board
    }

    pub fn file<T: Into<FileLetter> + Clone>(&self, letter: &T) -> Option<&File> {
        self.files
            .iter()
            .find(|file| file.letter == letter.clone().into())
    }

    pub fn file_mut<T: Into<FileLetter> + Clone>(&mut self, letter: &T) -> Option<&mut File> {
        self.files
            .iter_mut()
            .find(|file| file.letter == letter.clone().into())
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

        // 1. check moves
        let moves = piece_from.get_moves(self);

        if !moves.contains(mv) {
            return Err("Invalid move".into());
        }

        // 2. move piece
        let square_to = self
            .square_mut(&mv.to)
            .to_result("Move \"to\" invalid".into())?;

        piece_from.set_position(mv.to.clone());
        square_to.set_piece(piece_from);

        // 3. remove piece from old position
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
        Self {
            files: self.files.to_vec(),
        }
    }
}
