pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn get_opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            Color::White => "White".to_string(),
            Color::Black => "Black".to_string(),
        }
    }

    pub fn get_char(&self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }

    pub fn flip(&mut self) {
        *self = self.other();
    }

    pub fn other(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
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

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Color::White, Color::White) | (Color::Black, Color::Black)
        )
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        match self {
            Color::White => Color::White,
            Color::Black => Color::Black,
        }
    }
}

impl Copy for Color {}
