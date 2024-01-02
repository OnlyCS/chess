use crate::{movegen, prelude::*};

#[derive(Clone, Copy)]
pub enum SpecialMoveType {
    EnPassant,
    PawnDouble,
    Promo,
    CastleQueen,
    CastleKing,
}

#[derive(Clone)]
pub struct Position {
    pub n_white: Bitboard,
    pub n_black: Bitboard,

    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queens: Bitboard,
    pub kings: Bitboard,

    pub turn: Color,
    pub ep_target: Option<Square>,
    pub castling_rights: CastlingRights,
    pub last_pos: Option<Box<Position>>,
}

impl Position {
    pub fn new() -> Self {
        let n_white = Bitboard::rank(0) | Bitboard::rank(1);
        let n_black = Bitboard::rank(6) | Bitboard::rank(7);
        let backranks = Bitboard::rank(0) | Bitboard::rank(7);

        let rooks = backranks & (Bitboard::file(0) | Bitboard::file(7));
        let knights = backranks & (Bitboard::file(1) | Bitboard::file(6));
        let bishops = backranks & (Bitboard::file(2) | Bitboard::file(5));
        let queens = backranks & Bitboard::file(3);
        let kings = backranks & Bitboard::file(4);

        let pawns = Bitboard::rank(1) | Bitboard::rank(6);

        Position {
            n_white,
            n_black,
            pawns,
            knights,
            bishops,
            rooks,
            queens,
            kings,
            turn: Color::White,
            ep_target: None,
            castling_rights: CastlingRights::new(),
            last_pos: None,
        }
    }

    fn piece_at(&self, square: Square) -> Option<gui::PieceType> {
        if self.pawns & square.to_bitboard() != Bitboard::EMPTY {
            return Some(gui::PieceType::Pawn);
        }
        if self.knights & square.to_bitboard() != Bitboard::EMPTY {
            return Some(gui::PieceType::Knight);
        }
        if self.bishops & square.to_bitboard() != Bitboard::EMPTY {
            return Some(gui::PieceType::Bishop);
        }
        if self.rooks & square.to_bitboard() != Bitboard::EMPTY {
            return Some(gui::PieceType::Rook);
        }
        if self.queens & square.to_bitboard() != Bitboard::EMPTY {
            return Some(gui::PieceType::Queen);
        }
        if self.kings & square.to_bitboard() != Bitboard::EMPTY {
            return Some(gui::PieceType::King);
        }

        None
    }

    fn color_at(&self, square: Square) -> Option<Color> {
        if self.n_white & square.to_bitboard() != Bitboard::EMPTY {
            return Some(Color::White);
        }
        if self.n_black & square.to_bitboard() != Bitboard::EMPTY {
            return Some(Color::Black);
        }

        None
    }

    fn full_piece_at(&self, square: Square) -> Option<gui::Piece> {
        let color = self.color_at(square)?;
        let kind = self.piece_at(square)?;

        Some(gui::Piece { color, kind })
    }

    pub fn collect(&self) -> Vec<Option<gui::Piece>> {
        let mut pieces = vec![];

        for square in Square::every() {
            pieces.push(self.full_piece_at(square));
        }

        pieces
    }

    fn pieces_of_col_mut(&mut self, color: Color) -> &mut Bitboard {
        match color {
            Color::White => &mut self.n_white,
            Color::Black => &mut self.n_black,
        }
    }

    fn pieces_of_col(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.n_white,
            Color::Black => self.n_black,
        }
    }

    fn pieces_of_type_mut(&mut self, piece_type: gui::PieceType) -> &mut Bitboard {
        match piece_type {
            gui::PieceType::Pawn => &mut self.pawns,
            gui::PieceType::Knight => &mut self.knights,
            gui::PieceType::Bishop => &mut self.bishops,
            gui::PieceType::Rook => &mut self.rooks,
            gui::PieceType::Queen => &mut self.queens,
            gui::PieceType::King => &mut self.kings,
        }
    }

    fn try_find_special(&self, from: Square, to: Square) -> Option<SpecialMoveType> {
        let piece = self.piece_at(from)?;

        match piece {
            gui::PieceType::Pawn => {
                if from.distance_to(to) >= 2.0 {
                    Some(SpecialMoveType::PawnDouble)
                } else if self.ep_target.is_some_and(|a| a == to) {
                    Some(SpecialMoveType::EnPassant)
                } else if to.rank() == 0 || to.rank() == 7 {
                    Some(SpecialMoveType::Promo)
                } else {
                    None
                }
            }
            gui::PieceType::King => {
                if from.distance_to(to) >= 2.0 {
                    if from.file() > to.file() {
                        Some(SpecialMoveType::CastleQueen)
                    } else {
                        Some(SpecialMoveType::CastleKing)
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn _move(&mut self, from: Square, to: Square) -> Option<()> {
        let piece = self.piece_at(from)?;
        let color = self.color_at(from)?;

        for piece_type in gui::PieceType::every() {
            if piece == piece_type {
                *self.pieces_of_type_mut(piece_type) &= !from.to_bitboard();
                *self.pieces_of_type_mut(piece_type) |= to.to_bitboard();
                continue;
            }

            // any captures
            *self.pieces_of_type_mut(piece_type) &= !to.to_bitboard();
        }

        *self.pieces_of_col_mut(color) &= !from.to_bitboard();
        *self.pieces_of_col_mut(color) |= to.to_bitboard();
        *self.pieces_of_col_mut(color.other()) &= !to.to_bitboard();

        Some(())
    }

    /// returns: make a promotion
    pub fn make_move(&mut self, from: Square, to: Square) -> Option<bool> {
        let prev = self.clone();

        let color = self.color_at(from)?;
        let piece = self.piece_at(from)?;
        let special = self.try_find_special(from, to);

        if color != self.turn {
            return None;
        }

        self._move(from, to)?;

        self.turn.swap();
        self.last_pos = Some(Box::new(prev));
        self.ep_target = None;

        if let Some(special) = special {
            match special {
                SpecialMoveType::CastleKing => {
                    let rook_from = Square::new(to.rank(), 7);
                    let rook_to = Square::new(to.rank(), 5);

                    self._move(rook_from, rook_to)?;
                }
                SpecialMoveType::CastleQueen => {
                    let rook_from = Square::new(to.rank(), 0);
                    let rook_to = Square::new(to.rank(), 3);

                    self._move(rook_from, rook_to)?;
                }
                SpecialMoveType::PawnDouble => {
                    self.ep_target = Some(to.try_add(
                        match color {
                            Color::White => -1,
                            Color::Black => 1,
                        },
                        0,
                    )?);
                }
                SpecialMoveType::Promo => {
                    return Some(true);
                }
                SpecialMoveType::EnPassant => {
                    let kill_pawn = to
                        .try_add(
                            match color {
                                Color::White => -1,
                                Color::Black => 1,
                            },
                            0,
                        )?
                        .to_bitboard();

                    *self.pieces_of_type_mut(gui::PieceType::Pawn) &= !kill_pawn;
                    *self.pieces_of_col_mut(color.other()) &= !kill_pawn;
                }
            }
        }

        if piece == gui::PieceType::King {
            if color == Color::White {
                self.castling_rights.kingside_white = false;
                self.castling_rights.queenside_white = false;
            } else {
                self.castling_rights.kingside_black = false;
                self.castling_rights.queenside_black = false;
            }
        }

        if piece == gui::PieceType::Rook {
            if color == Color::White {
                if from == Square::new(0, 0) {
                    self.castling_rights.queenside_white = false;
                }
                if from == Square::new(0, 7) {
                    self.castling_rights.kingside_white = false;
                }
            } else {
                if from == Square::new(7, 0) {
                    self.castling_rights.queenside_black = false;
                }
                if from == Square::new(7, 7) {
                    self.castling_rights.kingside_black = false;
                }
            }
        }

        Some(false)
    }

    pub fn undo_move(&mut self) {
        if let Some(prev) = self.last_pos.take() {
            *self = *prev;
        }
    }

    pub fn in_check(&self, color: Color) -> bool {
        let king = self.kings & self.pieces_of_col(color);
        let square = king.last_bit();
        let occupied = self.n_black | self.n_white;

        let king_as_rook = movegen::rook(square, occupied);
        let king_as_bishop = movegen::bishop(square, occupied);
        let king_as_knight = movegen::knight(square);
        let king_as_pawn = movegen::pawn(square, occupied, color, self.ep_target);

        let check_rook = self.rooks | self.queens;
        let check_bishop = self.bishops | self.queens;

        let rook_check = king_as_rook & check_rook & self.pieces_of_col(color.other());
        let bishop_check = king_as_bishop & check_bishop & self.pieces_of_col(color.other());
        let knight_check = king_as_knight & self.knights & self.pieces_of_col(color.other());
        let pawn_check = king_as_pawn & self.pawns & self.pieces_of_col(color.other());

        let in_check = rook_check | bishop_check | knight_check | pawn_check;

        in_check != Bitboard::EMPTY
    }

    fn filter_checks(&self, mut moves: Bitboard, from: Square) -> Bitboard {
        let mut this = self.clone();

        for to_square in (*&moves).bit_pos_iter() {
            this.make_move(from, to_square);

            if this.in_check(self.turn) {
                moves &= !to_square.to_bitboard();
            }

            this.undo_move();
        }

        moves
    }

    pub fn moves_of(&self, pos: Square) -> Bitboard {
        let Some(color) = self.color_at(pos) else {
            return Bitboard::EMPTY;
        };

        if color != self.turn {
            return Bitboard::EMPTY;
        }

        let occupied = self.n_black | self.n_white;

        let moves = match self.piece_at(pos) {
            Some(gui::PieceType::Pawn) => movegen::pawn(pos, occupied, color, self.ep_target),
            Some(gui::PieceType::Knight) => movegen::knight(pos),
            Some(gui::PieceType::Bishop) => movegen::bishop(pos, occupied),
            Some(gui::PieceType::Rook) => movegen::rook(pos, occupied),
            Some(gui::PieceType::Queen) => movegen::queen(pos, occupied),
            Some(gui::PieceType::King) => movegen::king(pos, self.castling_rights, color, &self),
            None => Bitboard::EMPTY,
        };

        self.filter_checks(moves & !self.pieces_of_col(color), pos)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
