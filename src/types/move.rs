use crate::types::coordinate::Coordinate;

pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
    pub is_take: bool,
}

impl Move {
    pub fn new(from: Coordinate, to: Coordinate, is_take: bool) -> Move {
        Move { from, to, is_take }
    }

    // shouldn't implicitly clone, but can copy
    pub fn copy(&self) -> Self {
        Move {
            from: self.from.copy(),
            to: self.to.copy(),
            is_take: self.is_take,
        }
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to && self.is_take == other.is_take
    }
}