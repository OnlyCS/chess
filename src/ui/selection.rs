use crate::parts::position::Position;

pub enum SelectionType {
    Hover,
    Selected,
    Available,
}

#[derive(Clone)]
pub enum SelectionMode {
    SelectPiece,
    SelectMove,
}

impl Default for SelectionType {
    fn default() -> Self {
        Self::Hover
    }
}

pub struct Selection {
    pub hover: Position,
    pub selected: Option<Position>,
    pub avaliable: Vec<Position>,
}
