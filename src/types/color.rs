pub enum Color {
    White,
    Black,
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
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (Color::White, Color::White) | (Color::Black, Color::Black))
    }
}

impl Eq for Color {}

impl Clone for Color {
    fn clone(&self) -> Self {
        match self {
            Color::White => Color::White,
            Color::Black => Color::Black,
        }
    }
}