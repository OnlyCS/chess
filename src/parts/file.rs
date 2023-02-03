use std::error::Error;

use crate::types::file_letter::FileLetter;

use super::{position::Position, square::Square};

pub struct File {
    pub letter: FileLetter,
    squares: Vec<Square>,
}

impl File {
    pub fn new<T: Into<FileLetter>>(letter: T) -> Result<Self, Box<dyn Error>> {
        let letter: FileLetter = letter.into();
        let mut squares = Vec::new();

        for rank in 1..9 {
            let position = Position::new(letter.clone(), rank);
            squares.push(Square::new(position));
        }

        Ok(Self { letter, squares })
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
