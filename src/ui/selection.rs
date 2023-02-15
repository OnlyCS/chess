use crate::{parts::position::Position, types::r#move::Move};

#[derive(Clone, Debug)]
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

#[derive(Clone, Default)]
pub struct Selection {
    pub hover: Position,
    pub selected: Option<Position>,
    pub avaliable: Vec<Move>,
}

impl Selection {
    pub fn has(&self, pos: &Position) -> Option<SelectionType> {
        if self.hover == *pos {
            Some(SelectionType::Hover)
        } else if let Some(selected) = &self.selected {
            if selected == pos {
                Some(SelectionType::Selected)
            } else if self
                .avaliable
                .iter()
                .map(|x| x.clone().to)
                .collect::<Vec<_>>()
                .contains(pos)
            {
                Some(SelectionType::Available)
            } else {
                None
            }
        } else if self
            .avaliable
            .iter()
            .map(|x| x.clone().to)
            .collect::<Vec<_>>()
            .contains(pos)
        {
            Some(SelectionType::Available)
        } else {
            None
        }
    }
}
