use crate::core::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct CastleData {
    pub queenside: bool,
    pub kingside: bool,
    pub color: Color,
}

impl CastleData {
    pub fn fen(&self) -> String {
        match self.color {
            Color::White => match (self.queenside, self.kingside) {
                (true, true) => "KQ".to_string(),
                (true, false) => "Q".to_string(),
                (false, true) => "K".to_string(),
                (false, false) => "-".to_string(),
            },
            Color::Black => match (self.queenside, self.kingside) {
                (true, true) => "kq".to_string(),
                (true, false) => "q".to_string(),
                (false, true) => "k".to_string(),
                (false, false) => "-".to_string(),
            },
        }
    }
}
