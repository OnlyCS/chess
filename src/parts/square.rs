use crate::pieces::piece::Piece;

use super::position::Position;

pub struct Square {
    position: Position,
    piece: Option<Box<dyn Piece + Sync + Send>>,
}

impl Square {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            piece: None,
        }
    }

    pub fn set_piece(&mut self, piece: Box<dyn Piece + Sync + Send>) {
        self.piece = Some(piece);
    }

    pub fn get_piece(&self) -> Option<&Box<dyn Piece + Sync + Send>> {
        self.piece.as_ref()
    }

    pub fn get_piece_mut(&mut self) -> Option<&mut Box<dyn Piece + Sync + Send>> {
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
