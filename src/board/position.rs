use crate::{movegen, prelude::*};

#[derive(Clone, Copy)]
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
        }
    }

    pub fn piece_at(&self, square: Square) -> Option<gui::PieceType> {
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

    pub fn color_at(&self, square: Square) -> Option<Color> {
        if self.n_white & square.to_bitboard() != Bitboard::EMPTY {
            return Some(Color::White);
        }
        if self.n_black & square.to_bitboard() != Bitboard::EMPTY {
            return Some(Color::Black);
        }

        None
    }

    pub fn full_piece_at(&self, square: Square) -> Option<gui::Piece> {
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

    pub fn pieces_of_col(&self, color: Color) -> Bitboard {
        match color {
            Color::White => self.n_white,
            Color::Black => self.n_black,
        }
    }

    pub fn pieces_of_type(&mut self, piece_type: gui::PieceType) -> &mut Bitboard {
        match piece_type {
            gui::PieceType::Pawn => &mut self.pawns,
            gui::PieceType::Knight => &mut self.knights,
            gui::PieceType::Bishop => &mut self.bishops,
            gui::PieceType::Rook => &mut self.rooks,
            gui::PieceType::Queen => &mut self.queens,
            gui::PieceType::King => &mut self.kings,
        }
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> Option<()> {
        let piece = self.piece_at(from)?;
        let color = self.color_at(from)?;

        if color != self.turn {
            return None;
        }

        for piece_type in gui::PieceType::every() {
            if piece == piece_type {
                *self.pieces_of_type(piece_type) &= !from.to_bitboard();
                *self.pieces_of_type(piece_type) |= to.to_bitboard();
                continue;
            }

            // any captures
            *self.pieces_of_type(piece_type) &= !to.to_bitboard();
        }

        match color {
            Color::White => {
                self.n_white &= !from.to_bitboard();
                self.n_white |= to.to_bitboard();
                self.n_black &= !to.to_bitboard();
            }
            Color::Black => {
                self.n_black &= !from.to_bitboard();
                self.n_black |= to.to_bitboard();
                self.n_white &= !to.to_bitboard();
            }
        }

        self.turn.swap();

        Some(())
    }

    pub fn moves_of(&self, pos: Square) -> Bitboard {
        let Some(color) = self.color_at(pos) else {
            return Bitboard::EMPTY;
        };

        if color != self.turn {
            return Bitboard::EMPTY;
        }

        let moves = match self.piece_at(pos) {
            Some(gui::PieceType::Pawn) => {
                movegen::pawn(pos, self.n_black | self.n_white, color, self.ep_target)
            }
            Some(gui::PieceType::Knight) => movegen::knight(pos),
            Some(gui::PieceType::Bishop) => movegen::bishop(pos, self.n_black | self.n_white),
            Some(gui::PieceType::Rook) => movegen::rook(pos, self.n_black | self.n_white),
            Some(gui::PieceType::Queen) => movegen::queen(pos, self.n_black | self.n_white),
            Some(gui::PieceType::King) => movegen::king(pos, self.castling_rights, color),
            None => Bitboard::EMPTY,
        };

        moves & !self.pieces_of_col(color)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}
