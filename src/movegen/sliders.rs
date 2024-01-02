use crate::prelude::*;
use magic::{generated_lookups::*, generated_magics::*};

/// Get the rook moves in a position
///
/// * `rook_at` - The square the rook is at
/// * `occupied` - The bitboard of all occupied squares
/// * returns - The bitboard of all squares the rook can move to,
/// assuming occupied squares are enemy pieces
///
/// Make sure to bitwise-and the result with !friendly_pieces
pub fn rook(rook_at: Square, occupied: Bitboard) -> Bitboard {
    let rook_at = rook_at as usize;

    let movements = MOVEMENTS_ROOK[rook_at];
    let blockers = movements & occupied;
    let bits = movements.count_bits();
    let magic_index = blockers.trimmed_mul(MAGICS_ROOK[rook_at]) >> (64 - bits);
    let attacks = ATTACKS_ROOK[rook_at][magic_index as usize];

    return attacks;
}

/// Get the bishop moves in a position
///
/// * `bishop_at` - The square the bishop is at
/// * `occupied` - The bitboard of all occupied squares
/// * returns - The bitboard of all squares the bishop can move to,
/// assuming occupied squares are enemy pieces
///
/// Make sure to bitwise-and the result with !friendly_pieces
pub fn bishop(bishop_at: Square, occupied: Bitboard) -> Bitboard {
    let bishop_at = bishop_at as usize;

    let movements = MOVEMENTS_BISHOP[bishop_at];
    let blockers = movements & occupied;
    let bits = movements.count_bits();
    let magic_index = blockers.trimmed_mul(MAGICS_BISHOP[bishop_at]) >> (64 - bits);
    let attacks = ATTACKS_BISHOP[bishop_at][magic_index as usize];

    return attacks;
}

/// Get the queen moves in a position
///
/// * `queen_at` - The square the queen is at
/// * `occupied` - The bitboard of all occupied squares
/// * returns - The bitboard of all squares the queen can move to,
/// assuming occupied squares are enemy pieces
///
/// Make sure to bitwise-and the result with !friendly_pieces
pub fn queen(queen_at: Square, occupied: Bitboard) -> Bitboard {
    return rook(queen_at, occupied) | bishop(queen_at, occupied);
}
