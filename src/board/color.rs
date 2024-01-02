use core::fmt;

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
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "white"),
            Color::Black => write!(f, "black"),
        }
    }
}
