use crate::pieces::piece::Piece;

use super::position::Position;

pub struct Square<P: Piece> {
    position: Position,
    piece: Option<P>,
}

impl<P: Piece> Square<P> {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            piece: None,
        }
    }

    pub fn set_piece(&mut self, piece: P) {
        self.piece = Some(piece);
    }

    pub fn get_piece(&self) -> Option<&P> {
        self.piece.as_ref()
    }

    pub fn get_piece_mut(&mut self) -> Option<&mut P> {
        self.piece.as_mut()
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn is_empty(&self) -> bool {
        self.piece.is_none()
    }

    pub fn clear(&mut self) {
        self.piece = None;
    }
}

impl Clone for Square {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            piece: self.piece.clone(),
        }
    }
}
