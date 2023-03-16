use anyhow::Result;

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

pub struct Board<T: Piece> {
    files: Vec<File<T>>,
}

impl<T: Piece> Board<T> {
    pub fn new() -> Board<T> {
        let mut board = Self { files: Vec::new() };
        for letter in FileLetter::vec_all() {
            board.files.push(File::new(letter));
        }

        board
    }

    pub fn file<K: Into<FileLetter> + Clone>(&self, letter: &K) -> Option<&File<T>> {
        self.files
            .iter()
            .find(|file| file.letter == letter.clone().into())
    }

    pub fn file_mut<K: Into<FileLetter> + Clone>(&mut self, letter: &K) -> Option<&mut File<T>> {
        self.files
            .iter_mut()
            .find(|file| file.letter == letter.clone().into())
    }

    pub fn square(&self, position: &Position) -> Option<&Square<T>> {
        self.file(&position.file)?.rank(position.rank)
    }

    pub fn square_mut(&mut self, position: &Position) -> Option<&mut Square<T>> {
        self.file_mut(&position.file)?.rank_mut(position.rank)
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<()> {
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

    pub fn get_files(&self) -> &Vec<File<T>> {
        &self.files
    }

    pub fn get_squares(&self) -> Vec<&Square<T>> {
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

    pub fn get_squares_mut(&mut self) -> Vec<&mut Square<T>> {
        self.files
            .iter_mut()
            .flat_map(|x| x.get_squares_mut())
            .collect()
    }

    pub fn get_pieces(&self) -> Vec<&T> {
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

impl<T: Piece> Default for Board<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Piece> Clone for Board<T> {
    fn clone(&self) -> Self {
        Self {
            files: self.files.to_vec(),
        }
    }
}
