use crate::types::file_letter::FileLetter;

pub struct Position {
    pub file: FileLetter,
    pub rank: u8,
}

impl Position {
    pub fn new<T: Into<FileLetter>>(file: T, rank: u8) -> Self {
        Self {
            file: file.into(),
            rank,
        }
    }

    pub fn copy(&self) -> Self {
        Self::new(self.file.clone(), self.rank)
    }

    pub fn is_oob(&self) -> bool {
        self.file < FileLetter::A || self.file > FileLetter::H || self.rank < 1 || self.rank > 8
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
