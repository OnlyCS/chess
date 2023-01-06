use crate::types::*;

pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
    pub is_take: bool,
}

impl Move {
    pub fn new(from: Coordinate, to: Coordinate, is_take: bool) -> Move {
        Move { from, to, is_take }
    }
}