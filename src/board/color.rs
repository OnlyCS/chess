#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn other(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn swap(&mut self) {
        *self = self.other();
    }

    pub fn is_white(&self) -> bool {
        *self == Color::White
    }

    pub fn is_black(&self) -> bool {
        !self.is_white()
    }
}
