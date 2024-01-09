use crate::prelude::{gui::PieceType, *};

#[rustfmt::skip]
pub const PAWN: [i8; 64] = [
	 0,   0,   0,   0,   0,   0,   0,   0,
	50,  50,  50,  50,  50,  50,  50,  50,
	10,  10,  20,  30,  30,  20,  10,  10,
	 5,   5,  10,  25,  25,  10,   5,   5,
	 0,   0,   0,  20,  20,   0,   0,   0,
	 5,  -5, -10,   0,   0, -10,  -5,   5,
	 5,  10,  10, -20, -20,  10,  10,   5,
	 0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
pub const PAWN_ENDGAME: [i8; 64] = [
	 0,   0,   0,   0,   0,   0,   0,   0,
	80,  80,  80,  80,  80,  80,  80,  80,
	50,  50,  50,  50,  50,  50,  50,  50,
	30,  30,  30,  30,  30,  30,  30,  30,
	20,  20,  20,  20,  20,  20,  20,  20,
	10,  10,  10,  10,  10,  10,  10,  10,
	10,  10,  10,  10,  10,  10,  10,  10,
	 0,   0,   0,   0,   0,   0,   0,   0,
];

#[rustfmt::skip]
pub const ROOK: [i8; 64] = [
	 0,  0,   0,   0,   0,   0,   0,   0,
	 5, 10,  10,  10,  10,  10,  10,   5,
	-5,  0,   0,   0,   0,   0,   0,  -5,
	-5,  0,   0,   0,   0,   0,   0,  -5,
	-5,  0,   0,   0,   0,   0,   0,  -5,
	-5,  0,   0,   0,   0,   0,   0,  -5,
	-5,  0,   0,   0,   0,   0,   0,  -5,
	 0,  0,   0,   5,   5,   0,   0,   0,
];

#[rustfmt::skip]
pub const BISHOP: [i8; 64] = [
	-20, -10, -10, -10, -10, -10, -10, -20,
	-10,   0,   0,   0,   0,   0,   0, -10,
	-10,   0,   5,  10,  10,   5,   0, -10,
	-10,   5,   5,  10,  10,   5,   5, -10,
	-10,   0,  10,  10,  10,  10,   0, -10,
	-10,  10,  10,  10,  10,  10,  10, -10,
	-10,   5,   0,   0,   0,   0,   5, -10,
	-20, -10, -10, -10, -10, -10, -10, -20,
];

#[rustfmt::skip]
pub const KNIGHT: [i8; 64] = [
	-50, -40, -30, -30, -30, -30, -40, -50,
	-40, -20,   0,   0,   0,   0, -20, -40,
	-30,   0,  10,  15,  15,  10,   0, -30,
	-30,   5,  15,  20,  20,  15,   5, -30,
	-30,   0,  15,  20,  20,  15,   0, -30,
	-30,   5,  10,  15,  15,  10,   5, -30,
	-40, -20,   0,   5,   5,   0, -20, -40,
	-50, -40, -30, -30, -30, -30, -40, -50,
];

#[rustfmt::skip]
pub const KING: [i8; 64] = [
	-30, -40, -40, -50, -50, -40, -40, -30,
	-30, -40, -40, -50, -50, -40, -40, -30,
	-30, -40, -40, -50, -50, -40, -40, -30,
	-30, -40, -40, -50, -50, -40, -40, -30,
	-20, -30, -30, -40, -40, -30, -30, -20,
	-10, -20, -20, -20, -20, -20, -20, -10,
	 20,  20,   0,   0,   0,   0,  20,  20,
	 20,  30,  10,   0,   0,  10,  30,  20,
];

#[rustfmt::skip]
pub const KING_ENDGAME: [i8; 64] = [
	-50, -40, -30, -20, -20, -30, -40, -50,
	-30, -20, -10,   0,   0, -10, -20, -30,
	-30, -10,  20,  30,  30,  20, -10, -30,
	-30, -10,  30,  40,  40,  30, -10, -30,
	-30, -10,  30,  40,  40,  30, -10, -30,
	-30, -10,  20,  30,  30,  20, -10, -30,
	-30, -30,   0,   0,   0,   0, -30, -30,
	-50, -30, -30, -30, -30, -30, -30, -50,
];

#[rustfmt::skip]
pub const QUEEN: [i8; 64] = [
	-20, -10, -10,  -5,  -5, -10, -10, -20,
	-10,   0,   0,   0,   0,   0,   0, -10,
	-10,   0,   5,   5,   5,   5,   0, -10,
	 -5,   0,   5,   5,   5,   5,   0,  -5,
	  0,   0,   5,   5,   5,   5,   0,  -5,
	-10,   5,   5,   5,   5,   5,   0, -10,
	-10,   0,   5,   0,   0,   0,   0, -10,
	-20, -10, -10,  -5,  -5, -10, -10, -20,
];

#[derive(Clone, Copy)]
pub struct PieceTableEvaluator;

impl PieceTableEvaluator {
    pub fn eval_white(position: &Position) -> f64 {
        let mut score = 0.0;

        for square in position.n_white.bit_pos_iter() {
            let piece_at = position.piece_at(square);
            let ev_square = square as usize;
            let endgame = position.pieces().count_ones() <= 7;

            score += match piece_at {
                Some(PieceType::King) if endgame => KING_ENDGAME[ev_square],
                Some(PieceType::Pawn) if endgame => PAWN_ENDGAME[ev_square],
                Some(PieceType::Pawn) => PAWN[ev_square],
                Some(PieceType::Rook) => ROOK[ev_square],
                Some(PieceType::Knight) => KNIGHT[ev_square],
                Some(PieceType::Bishop) => BISHOP[ev_square],
                Some(PieceType::Queen) => QUEEN[ev_square],
                Some(PieceType::King) => KING[ev_square],
                _ => 0,
            } as f64;
        }

        score * 0.1
    }

    pub fn eval_black(position: &Position) -> f64 {
        let mut score = 0.0;

        for square in position.n_black.bit_pos_iter() {
            let piece_at = position.piece_at(square);
            let ev_square = Square::new(square.file(), 7 - square.rank()) as usize;
            let endgame = position.pieces().count_ones() <= 7;

            score += match piece_at {
                Some(PieceType::King) if endgame => KING_ENDGAME[ev_square],
                Some(PieceType::Pawn) if endgame => PAWN_ENDGAME[ev_square],
                Some(PieceType::Pawn) => PAWN[ev_square],
                Some(PieceType::Rook) => ROOK[ev_square],
                Some(PieceType::Knight) => KNIGHT[ev_square],
                Some(PieceType::Bishop) => BISHOP[ev_square],
                Some(PieceType::Queen) => QUEEN[ev_square],
                Some(PieceType::King) => KING[ev_square],
                _ => 0,
            } as f64;
        }

        score * 0.1
    }
}

impl StaticEvaluator for PieceTableEvaluator {
    fn eval(&self, position: &Position) -> f64 {
        Self::eval_white(position) - Self::eval_black(position)
    }
}
