use anyhow::{ensure, Result};

use super::file::FileLetter;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
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
        Self::new(self.file, self.rank)
    }

    pub fn is_oob(&self) -> bool {
        self.file < FileLetter::A || self.file > FileLetter::H || self.rank < 1 || self.rank > 8
    }

    pub fn up(&self, ct: i32) -> Result<Self> {
        let rank = self.rank as i32 + ct;

        ensure!(rank <= 8, "Cannot go up from rank 8");

        Ok(Self::new(self.file, rank as u8))
    }

    pub fn up_loop(&self, ct: i32) -> Self {
        let mut rank = self.rank as i32 + ct;

        while rank > 8 {
            rank -= 8;
        }

        Self::new(self.file, rank as u8)
    }

    pub fn down(&self, ct: i32) -> Result<Self> {
        let rank = self.rank as i32 - ct;

        ensure!(rank >= 1, "Cannot go down from rank 1");

        Ok(Self::new(self.file, rank as u8))
    }

    pub fn down_loop(&self, ct: i32) -> Self {
        let mut rank = self.rank as i32 - ct;

        while rank < 1 {
            rank += 8;
        }

        Self::new(self.file, rank as u8)
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
            return Ok(*self);
        }

        Self::right_recursive(*self, ct)
    }

    pub fn right_loop(&self, ct: i32) -> Self {
        Self::right_recursive_loop(*self, ct)
    }

    pub fn left(&self, ct: i32) -> Result<Self> {
        if ct == 0 {
            return Ok(*self);
        }

        Self::left_recursive(*self, ct)
    }

    pub fn left_loop(&self, ct: i32) -> Self {
        Self::left_recursive_loop(*self, ct)
    }

    pub fn fen(&self) -> String {
        format!("{}{}", self.file.fen(), self.rank)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new('a', 1)
    }
}
