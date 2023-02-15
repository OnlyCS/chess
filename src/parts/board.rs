use std::error::Error;

use crate::{
    pieces::piece::Piece,
    types::{
        color::Color,
        etc::ToResult,
        file_letter::FileLetter,
        piece_type::PieceType,
        r#move::{Move, MoveFilter},
    },
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

        for square in self.get_squares() {
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

    pub fn get_squares(&self) -> Vec<&Square> {
        let files = &self.files;
        let iter = files.iter();
        let mut squares = Vec::with_capacity(64);

        for f in iter {
            for s in f.get_squares() {
                squares.push(s);
            }
        }

        squares
    }

    pub fn get_squares_mut(&mut self) -> Vec<&mut Square> {
        self.files
            .iter_mut()
            .flat_map(|x| x.get_squares_mut())
            .collect()
    }

    pub fn get_pieces(&self) -> Vec<&Box<dyn Piece + Send + Sync>> {
        self.get_squares()
            .iter()
            .filter_map(|x| x.get_piece())
            .collect()
    }

    pub fn is_check(&self, color: Color) -> bool {
        let working_board = self.clone();

        for mv in working_board.get_moves_for(color.get_opposite()) {
            let mut sub_working_board = working_board.clone();
            match sub_working_board.make_move(&mv) {
                Ok(_) => {}
                Err(_) => return true,
            };

            // count kings in board
            let kings = sub_working_board
                .get_pieces()
                .iter()
                .filter(|p| p.get_type() == PieceType::King)
                .count();

            if kings != 2 {
                return true;
            }
        }

        false
    }

    pub fn is_mate(&self, color: Color) -> bool {
        let mut moves = self.get_moves_for(color);
        moves.filter_king_check(self, color);

        moves.is_empty()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Self {
            files: self.files.to_vec(),
        }
    }
}
