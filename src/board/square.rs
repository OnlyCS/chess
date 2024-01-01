use std::ops::Range;

use crate::prelude::*;

#[const_trait]
pub trait SquareU8: Sized {
    fn new(rank: u8, file: u8) -> Self;
    fn rank(&self) -> u8;
    fn file(&self) -> u8;
    fn valid_square(&self) -> bool;
    fn to_bitboard(&self) -> Bitboard;
    fn try_add(&self, other_rank: i8, other_file: i8) -> Option<Self>;
    fn get(&self) -> u8;

    fn every() -> impl Iterator<Item = Self>;

    // fn pretty(&self) -> String {
    //     if !self.valid_square() {
    //         return String::from("Invalid square");
    //     }

    //     let file = { ('a' as u8 + self.file()) as char };
    //     let rank = { ('1' as u8 + self.rank()) as char };

    //     return format!("{}{}", file, rank);
    // }
}

/// Number 0-63
pub type Square = u8;

impl const SquareU8 for Square {
    fn new(rank: u8, file: u8) -> Square {
        rank * 8 + file
    }

    fn rank(&self) -> u8 {
        self / 8
    }

    fn file(&self) -> u8 {
        self % 8
    }

    fn valid_square(&self) -> bool {
        *self < 64
    }

    fn to_bitboard(&self) -> Bitboard {
        1 << self
    }

    fn try_add(&self, other_rank: i8, other_file: i8) -> Option<Square> {
        let rank = self.rank() as i8 + other_rank;
        let file = self.file() as i8 + other_file;

        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None;
        }

        return Some(Square::new(rank as u8, file as u8));
    }

    fn every() -> Range<Square> {
        0..64
    }

    fn get(&self) -> u8 {
        *self
    }
}
