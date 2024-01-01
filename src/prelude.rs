pub use crate::{
    board::{
        bitboard::{Bitboard, BitboardU64},
        square::{Square, SquareU8},
    },
    movegen::magic,
};

pub use rayon::prelude::*;
