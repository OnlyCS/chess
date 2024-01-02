use crate::prelude::*;

/// Get the pawn moves in a position
///
/// * `pawn_at` - The square the pawn is at
/// * `occupied` - The bitboard of all occupied squares
/// * `color` - The color of the pawn
/// * `ep_target` - The square of the en passant target, if any
/// * returns - The bitboard of all squares the pawn can move to,
/// assuming occupied squares are enemy pieces
///
/// Make sure to bitwise-and the result with !friendly_pieces
pub fn pawn(
    pawn_at: Square,
    occupied: Bitboard,
    color: Color,
    ep_target: Option<Square>,
) -> Bitboard {
    let pawn_bb = pawn_at.to_bitboard();
    let empty = !occupied;

    let mut moves = Bitboard::EMPTY;

    if color == Color::White {
        moves |= pawn_bb.safe_shl(8) & empty;
        moves |= pawn_bb.safe_shl(16) & empty.safe_shl(8) & empty & Bitboard::rank(3);
        moves |= pawn_bb.safe_shl(7) & occupied & Bitboard::rank(pawn_at.rank() + 1);
        moves |= pawn_bb.safe_shl(9) & occupied & Bitboard::rank(pawn_at.rank() + 1);

        if let Some(target) = ep_target
            && target.rank() == pawn_at.rank() + 1
        {
            let target_bb = target.to_bitboard();

            moves |= pawn_bb.safe_shl(7) & target_bb;
            moves |= pawn_bb.safe_shl(9) & target_bb;
        }
    } else {
        moves |= pawn_bb >> 8 & empty;
        moves |= pawn_bb >> 16 & empty >> 8 & empty & Bitboard::rank(4);
        moves |= pawn_bb >> 7 & occupied & Bitboard::rank(pawn_at.rank() - 1);
        moves |= pawn_bb >> 9 & occupied & Bitboard::rank(pawn_at.rank() - 1);

        if let Some(target) = ep_target
            && target.rank() == pawn_at.rank() - 1
        {
            let target_bb = target.to_bitboard();

            moves |= pawn_bb >> 7 & target_bb;
            moves |= pawn_bb >> 9 & target_bb;
        }
    }

    moves
}
