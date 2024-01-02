#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    pub kingside_white: bool,
    pub queenside_white: bool,
    pub kingside_black: bool,
    pub queenside_black: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        CastlingRights {
            kingside_white: true,
            queenside_white: true,
            kingside_black: true,
            queenside_black: true,
        }
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self::new()
    }
}
