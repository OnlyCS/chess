use crate::prelude::*;

const KING_MOVEMENTS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const KNIGHT_MOVEMENTS: [(i8, i8); 8] = [
    (-2, -1),
    (-2, 1),
    (-1, -2),
    (-1, 2),
    (1, -2),
    (1, 2),
    (2, -1),
    (2, 1),
];

/// Get the king moves in a position
///
/// * `king_at` - The square the king is at
/// * `castling` - The castling rights of the position
/// * `color` - The color of the king
/// * returns - The bitboard of all squares the king can move to,
/// assuming occupied squares are enemy pieces
///
/// Make sure to bitwise-and the result with !friendly_pieces
pub fn king(
    king_at: Square,
    castling: CastlingRights,
    color: Color,
    position: Position,
) -> Bitboard {
    let mut moves = Bitboard::EMPTY;

    for (file, rank) in KING_MOVEMENTS {
        if let Some(square) = king_at.try_add(file, rank) {
            moves |= square.to_bitboard();
        }
    }

    let in_check_ks = {
        let mut position = *&position;
        position.make_move(king_at, Square::new(king_at.rank(), 5));

        position.in_check(color)
    };

    let in_check_qs = {
        let mut position = *&position;
        position.make_move(king_at, Square::new(king_at.file(), 3));

        position.in_check(color)
    };

    let blocking_wks = 0x60 & position.occupied();
    let blocking_wqs = 0xc & position.occupied();
    let blocking_bks = 0x6000000000000000 & position.occupied();
    let blocking_bqs = 0xc00000000000000 & position.occupied();

    if !position.in_check(color) {
        match color {
            Color::White => {
                if castling.kingside_white && !in_check_ks && blocking_wks == Bitboard::EMPTY {
                    moves |= Square::new(0, 6).to_bitboard();
                }

                if castling.queenside_white && !in_check_qs && blocking_wqs == Bitboard::EMPTY {
                    moves |= Square::new(0, 2).to_bitboard();
                }
            }
            Color::Black => {
                if castling.kingside_black && !in_check_ks && blocking_bks == Bitboard::EMPTY {
                    moves |= Square::new(7, 6).to_bitboard();
                }
                if castling.queenside_black && !in_check_qs && blocking_bqs == Bitboard::EMPTY {
                    moves |= Square::new(7, 2).to_bitboard();
                }
            }
        }
    }

    moves
}

/// Get the knight moves in a position
///
/// * `knight_at` - The square the knight is at
/// * returns - The bitboard of all squares the knight can move to,
/// assuming occupied squares are enemy pieces
///
/// Make sure to bitwise-and the result with !friendly_pieces
pub fn knight(knight_at: Square) -> Bitboard {
    let mut moves = Bitboard::EMPTY;

    for (file, rank) in KNIGHT_MOVEMENTS {
        if let Some(square) = knight_at.try_add(file, rank) {
            moves |= square.to_bitboard();
        }
    }

    moves
}
