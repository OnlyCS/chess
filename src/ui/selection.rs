use crate::core::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum Selection {
    /// hovered square
    SelectPiece(Position),

    /// hovered square, selected piece, available moves
    SelectMove(Position, Position, Vec<Position>),
}

impl Selection {
    pub fn up(&mut self) {
        match self {
            Selection::SelectPiece(pos) => {
                *pos = pos.up(1).unwrap_or(*pos);
            }
            Selection::SelectMove(pos, _, _) => {
                *pos = pos.up(1).unwrap_or(*pos);
            }
        }
    }

    pub fn down(&mut self) {
        match self {
            Selection::SelectPiece(pos) => {
                *pos = pos.down(1).unwrap_or(*pos);
            }
            Selection::SelectMove(pos, _, _) => {
                *pos = pos.down(1).unwrap_or(*pos);
            }
        }
    }

    pub fn left(&mut self) {
        match self {
            Selection::SelectPiece(pos) => {
                *pos = pos.left(1).unwrap_or(*pos);
            }
            Selection::SelectMove(pos, _, _) => {
                *pos = pos.left(1).unwrap_or(*pos);
            }
        }
    }

    pub fn right(&mut self) {
        match self {
            Selection::SelectPiece(pos) => {
                *pos = pos.right(1).unwrap_or(*pos);
            }
            Selection::SelectMove(pos, _, _) => {
                *pos = pos.right(1).unwrap_or(*pos);
            }
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::SelectPiece(Position::default())
    }
}
