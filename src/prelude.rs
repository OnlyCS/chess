pub use crate::{
    board::{
        bitboard::{Bitboard, BitboardU64},
        color::Color,
        square::{Square, SquareU8},
    },
    movegen::magic,
};

pub use rayon::prelude::*;
