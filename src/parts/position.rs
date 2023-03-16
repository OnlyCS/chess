use anyhow::Result;

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

    pub fn up(&self, ct: i32) -> Result<Self> {
        let rank = self.rank as i32 + ct;

        if rank > 8 {
            return Err("Cannot go up from rank 8".into());
        }

        Ok(Self::new(self.file.clone(), rank as u8))
    }

    pub fn up_loop(&self, ct: i32) -> Self {
        let mut rank = self.rank as i32 + ct;

        while rank > 8 {
            rank -= 8;
        }

        Self::new(self.file.clone(), rank as u8)
    }

    pub fn down(&self, ct: i32) -> Result<Self> {
        let rank = self.rank as i32 - ct;

        if rank < 1 {
            return Err("Cannot go down from rank 1".into());
        }

        Ok(Self::new(self.file.clone(), rank as u8))
    }

    pub fn down_loop(&self, ct: i32) -> Self {
        let mut rank = self.rank as i32 - ct;

        while rank < 1 {
            rank += 8;
        }

        Self::new(self.file.clone(), rank as u8)
    }

    fn left_recursive(pos: Self, ct: i32) -> Result<Self> {
        if ct == 0 {
            return Ok(pos);
        }

        let file = pos.file.prev()?;
        let pos = Self::new(file, pos.rank);
        Self::left_recursive(pos, ct - 1)
    }

    fn left_recursive_loop(pos: Self, ct: i32) -> Self {
        if ct == 0 {
            return pos;
        }

        let file = pos.file.prev_loop();
        let pos = Self::new(file, pos.rank);
        Self::left_recursive_loop(pos, ct - 1)
    }

    fn right_recursive(pos: Self, ct: i32) -> Result<Self> {
        if ct == 0 {
            return Ok(pos);
        }

        let file = pos.file.next()?;
        let pos = Self::new(file, pos.rank);
        Self::right_recursive(pos, ct - 1)
    }

    fn right_recursive_loop(pos: Self, ct: i32) -> Self {
        if ct == 0 {
            return pos;
        }

        let file = pos.file.next_loop();
        let pos = Self::new(file, pos.rank);
        Self::right_recursive_loop(pos, ct - 1)
    }

    pub fn right(&self, ct: i32) -> Result<Self> {
        if ct == 0 {
            return Ok(self.clone());
        }

        Self::right_recursive(self.clone(), ct)
    }

    pub fn right_loop(&self, ct: i32) -> Self {
        Self::right_recursive_loop(self.clone(), ct)
    }

    pub fn left(&self, ct: i32) -> Result<Self> {
        if ct == 0 {
            return Ok(self.clone());
        }

        Self::left_recursive(self.clone(), ct)
    }

    pub fn left_loop(&self, ct: i32) -> Self {
        Self::left_recursive_loop(self.clone(), ct)
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
