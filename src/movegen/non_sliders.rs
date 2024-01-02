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
    position: &Position,
) -> Bitboard {
    let mut position = position.clone();
    let mut moves = Bitboard::EMPTY;

    for (file, rank) in KING_MOVEMENTS {
        if let Some(square) = king_at.try_add(file, rank) {
            moves |= square.to_bitboard();
        }
    }

    let in_check_ks = {
        position.make_move(king_at, Square::new(king_at.rank(), 5));
        let check = position.in_check(color);
        position.undo_move();

        check
    };

    let in_check_qs = {
        position.make_move(king_at, Square::new(king_at.file(), 3));
        let check = position.in_check(color);
        position.undo_move();

        check
    };

    if !position.in_check(color) {
        match color {
            Color::White => {
                if castling.kingside_white && !in_check_ks {
                    moves |= Square::new(0, 6).to_bitboard();
                }
                if castling.queenside_white && !in_check_qs {
                    moves |= Square::new(0, 2).to_bitboard();
                }
            }
            Color::Black => {
                if castling.kingside_black && !in_check_ks {
                    moves |= Square::new(7, 6).to_bitboard();
                }
                if castling.queenside_black && !in_check_qs {
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
