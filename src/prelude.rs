pub use crate::{
    board::{
        bitboard::{Bitboard, BitboardU64},
        castling::CastlingRights,
        color::Color,
        position::Position,
        square::{Square, SquareU8},
    },
    movegen::magic,
};

pub use rayon::prelude::*;

pub mod gui {
    pub use crate::gui::gui_piece::*;
}

pub use itertools::Itertools;
