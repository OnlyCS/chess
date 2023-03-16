use crate::{
    parts::{board::Board, position::Position},
    types::{
        color::Color,
        piece_type::PieceType,
        r#move::{Move, MoveModifier},
    },
};

use super::piece::Piece;

pub struct Knight {
    color: Color,
    position: Position,
}

impl Knight {
    pub fn new(color: Color, position: Position) -> Self {
        Self { color, position }
    }
}

impl Piece for Knight {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn get_type(&self) -> PieceType {
        PieceType::Knight
    }

    fn get_moves(&self, board: &Board) -> Vec<Move> {
        // let move_nums: Vec<[i32; 2]> = vec![
        //     [1, 2],
        //     [1, -2],
        //     [-1, 2],
        //     [-1, -2],
        //     [2, 1],
        //     [2, -1],
        //     [-2, 1],
        //     [-2, -1],
        // ];
        let move_nums = vec![
            self.position.up(1).map(|x| x.right(2)),
            self.position.up(1).map(|x| x.left(2)),
            self.position.down(1).map(|x| x.right(2)),
            self.position.down(1).map(|x| x.left(2)),
            self.position.up(2).map(|x| x.right(1)),
            self.position.up(2).map(|x| x.left(1)),
            self.position.down(2).map(|x| x.right(1)),
            self.position.down(2).map(|x| x.left(1)),
        ];

        let mut moves: Vec<Move> = Vec::new();

        for position in move_nums.into_iter().flatten().flatten() {
            let mut capture = false;
            let mut keep = true;

            if let Some(Some(piece)) = board.square(&position).map(|x| x.get_piece()) {
                if piece.get_color() != &self.color {
                    capture = true;
                } else {
                    keep = false;
                }
            }

            if keep {
                let mut modifiers = Vec::new();
                if capture {
                    modifiers.push(MoveModifier::Capture);
                }

                moves.push(Move::new(self.position.clone(), position, modifiers));
            }
        }

        moves
    }

    fn clone(&self) -> Box<dyn Piece + Sync + Send> {
        Box::new(Self {
            color: self.color,
            position: self.position.clone(),
        })
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }
}
