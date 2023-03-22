use super::{piece::Piece, position::Position};

#[derive(Debug, Clone)]
pub struct Square {
    position: Position,
    piece: Option<Piece>,
}

impl Square {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            piece: None,
        }
    }

    pub fn set_piece(&mut self, mut piece: Piece) {
        piece.set_position(self.position);

        self.piece = Some(piece);
    }

    pub fn get_piece(&self) -> Option<&Piece> {
        self.piece.as_ref()
    }

    pub fn get_piece_owned(&mut self) -> Option<Piece> {
        self.piece.take()
    }

    pub fn get_piece_mut(&mut self) -> Option<&mut Piece> {
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

    pub fn fen(&self) -> char {
        match &self.piece {
            Some(piece) => piece.fen(),
            None => '1',
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::new(Position::default())
    }
}
