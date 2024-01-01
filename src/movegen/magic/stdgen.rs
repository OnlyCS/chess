// standard move generation -- inefficient, which is why we precalc!

use crate::prelude::*;

/// up, down, left, right
pub const DIRS_ROOK: [(i8, i8); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

/// up-left, up-right, down-left, down-right
pub const DIRS_BISHOP: [(i8, i8); 4] = [(-1, 1), (1, 1), (-1, -1), (1, -1)];

/// Generate the movement mask
///
/// * `starting` - the square the piece is on
/// * `directions` - the directions the piece can move in (use the DIRS constants)
/// * returns - the movement mask
pub fn gen_movement_mask(starting: Square, directions: [(i8, i8); 4]) -> Bitboard {
    let mut bb = Bitboard::EMPTY;

    // no const-for is goofy ahh
    // FIXME: dont include last square optimization (somehow)
    for (file, rank) in directions {
        for dest in 1..7 {
            let next = dest + 1;

            let true = starting.try_add(file * next, rank * next).is_some() else {
                break;
            };

            let Some(current) = starting.try_add(file * dest, rank * dest) else {
                break;
            };

            bb.set(current);
        }
    }

    bb
}

/// Generate the attack mask
///
/// * `starting` - the square the piece is on
/// * `filled` - the bitboard of all pieces on the board. assumes all pieces are enemy pieces
/// * `directions` - the directions the piece can move in (use the DIRS constants)
/// * returns - the attack mask, assuming all pieces are enemy pieces
pub fn gen_attack_mask(starting: Square, filled: Bitboard, directions: [(i8, i8); 4]) -> Bitboard {
    let mut bb = Bitboard::EMPTY;

    for (file, rank) in directions {
        for dest in 1..8 {
            let Some(current) = starting.try_add(file * dest, rank * dest) else {
                break;
            };

            bb.set(current);

            if filled.at(current) {
                break;
            }
        }
    }

    bb
}
