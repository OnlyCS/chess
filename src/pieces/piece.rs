use crate::types::*;

pub(in crate::pieces) trait Piece {
    fn get_coords(&self) -> &Coordinate;
    fn get_color(&self) -> &Color;
    fn get_type(&self) -> PieceType;
    fn get_moves(&self, board: &[Vec<Box<dyn Piece>>]) -> Vec<Move>;

    fn get_coords_mut(&mut self) -> &mut Coordinate;

    fn is_empty(&self) -> bool {
        self.get_type() == PieceType::Empty
    }

    fn move_to(&mut self, to: Coordinate) {
        self.get_coords_mut().set(to.x, to.y);
    }
}
