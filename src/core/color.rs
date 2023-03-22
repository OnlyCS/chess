use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn fen(&self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }

    pub fn flip(&mut self) {
        *self = self.opposite();
    }
}

impl TryFrom<i32> for Color {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::White),
            1 => Ok(Color::Black),
            _ => Err("Invalid Color".into()),
        }
    }
}

impl From<bool> for Color {
    fn from(value: bool) -> Self {
        match value {
            true => Color::White,
            false => Color::Black,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}
