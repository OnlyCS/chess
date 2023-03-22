use std::fmt::{Display, Formatter};

use crate::core::{
    board::Board, color::Color, piece_move::Move, piece_move::MoveModifier, position::Position,
};

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn get_value(&self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 100,
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceType::Pawn => write!(f, "P"),
            PieceType::Knight => write!(f, "Kn"),
            PieceType::Bishop => write!(f, "B"),
            PieceType::Rook => write!(f, "R"),
            PieceType::Queen => write!(f, "Q"),
            PieceType::King => write!(f, "K"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    color: Color,
    position: Position,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, position: Position, piece_type: PieceType) -> Self {
        Self {
            color,
            position,
            piece_type,
        }
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn get_moves(&self, board: &Board) -> Vec<Move> {
        match self.piece_type {
            PieceType::Pawn => {
                let mut moves = Vec::new();

                // check a single move
                if let Ok(Some(square)) = match self.color {
                    Color::White => self.position.up(1).map(|i| board.square(i)),
                    Color::Black => self.position.down(1).map(|i| board.square(i)),
                } {
                    if square.is_empty() {
                        moves.push(Move {
                            from: self.position,
                            to: *square.get_position(),
                            modifiers: vec![],
                            color: self.color,
                            piece: self.piece_type,
                        });

                        // check a double move (only if single move available)
                        if match self.color {
                            Color::White => self.position.rank == 2,
                            Color::Black => self.position.rank == 7,
                        } {
                            if let Ok(Some(square)) = match self.color {
                                Color::White => self.position.up(2).map(|i| board.square(i)),
                                Color::Black => self.position.down(2).map(|i| board.square(i)),
                            } {
                                if square.is_empty() {
                                    moves.push(Move {
                                        from: self.position,
                                        to: *square.get_position(),
                                        modifiers: vec![MoveModifier::PawnDoubleMove],
                                        color: self.color,
                                        piece: self.piece_type,
                                    });
                                }
                            }
                        }
                    }
                }

                // check capture to right
                if let Ok(Ok(Some(square))) = match self.color {
                    Color::White => self
                        .position
                        .up(1)
                        .map(|i| i.right(1).map(|j| board.square(j))),
                    Color::Black => self
                        .position
                        .down(1)
                        .map(|i| i.right(1).map(|j| board.square(j))),
                } {
                    if let Some(piece) = square.get_piece() {
                        if piece.get_color() != self.get_color() {
                            moves.push(Move {
                                from: self.position,
                                to: *square.get_position(),
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    }
                }

                // check capture to left
                if let Ok(Ok(Some(square))) = match self.color {
                    Color::White => self
                        .position
                        .up(1)
                        .map(|i| i.left(1).map(|j| board.square(j))),
                    Color::Black => self
                        .position
                        .down(1)
                        .map(|i| i.left(1).map(|j| board.square(j))),
                } {
                    if let Some(piece) = square.get_piece() {
                        if piece.get_color() != self.get_color() {
                            moves.push(Move {
                                from: self.position,
                                to: *square.get_position(),
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    }
                }

                for m in moves.iter_mut() {
                    if m.to.rank == 8 && self.color == Color::White
                        || m.to.rank == 1 && self.color == Color::Black
                    {
                        m.modifiers.push(MoveModifier::PromotionUnknown(self.color));
                    }
                }

                moves
            }
            PieceType::Knight => {
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

                    if let Some(Some(piece)) = board.square(position).map(|x| x.get_piece()) {
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

                        moves.push(Move {
                            from: self.position,
                            to: position,
                            modifiers,
                            color: self.color,
                            piece: self.piece_type,
                        });
                    }
                }

                moves
            }
            PieceType::Bishop => {
                let mut moves = Vec::new();

                // top-right
                for i in 1..=8 {
                    let position = match self.position.up(i).map(|x| x.right(i)) {
                        Ok(Ok(position)) => position,
                        _ => break,
                    };

                    if let Some(square) = board.square(position) {
                        if let Some(piece) = square.get_piece() {
                            if piece.get_color() != &self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position,
                                    modifiers: vec![MoveModifier::Capture],
                                    color: self.color,
                                    piece: self.piece_type,
                                });
                            }

                            break;
                        } else {
                            moves.push(Move {
                                from: self.position,
                                to: position,
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    } else {
                        break;
                    }
                }

                // top-left
                for i in 1..=8 {
                    let position = match self.position.up(i).map(|x| x.left(i)) {
                        Ok(Ok(position)) => position,
                        _ => break,
                    };

                    if let Some(square) = board.square(position) {
                        if let Some(piece) = square.get_piece() {
                            if piece.get_color() != &self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position,
                                    modifiers: vec![MoveModifier::Capture],
                                    color: self.color,
                                    piece: self.piece_type,
                                });
                            }

                            break;
                        } else {
                            moves.push(Move {
                                from: self.position,
                                to: position,
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    } else {
                        break;
                    }
                }

                // bottom-right
                for i in 1..=8 {
                    let position = match self.position.down(i).map(|x| x.right(i)) {
                        Ok(Ok(position)) => position,
                        _ => break,
                    };

                    if let Some(square) = board.square(position) {
                        if let Some(piece) = square.get_piece() {
                            if piece.get_color() != &self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position,
                                    modifiers: vec![MoveModifier::Capture],
                                    color: self.color,
                                    piece: self.piece_type,
                                });
                            }

                            break;
                        } else {
                            moves.push(Move {
                                from: self.position,
                                to: position,
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    } else {
                        break;
                    }
                }

                // bottom-left
                for i in 1..=8 {
                    let position = match self.position.down(i).map(|x| x.left(i)) {
                        Ok(Ok(position)) => position,
                        _ => break,
                    };

                    if let Some(square) = board.square(position) {
                        if let Some(piece) = square.get_piece() {
                            if piece.get_color() != &self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position,
                                    modifiers: vec![MoveModifier::Capture],
                                    color: self.color,
                                    piece: self.piece_type,
                                });
                            }

                            break;
                        } else {
                            moves.push(Move {
                                from: self.position,
                                to: position,
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    } else {
                        break;
                    }
                }

                moves
            }
            PieceType::Rook => {
                let mut moves = Vec::new();

                //top
                for i in 1..=8 {
                    let position = match self.position.up(i) {
                        Ok(p) => p,
                        Err(_) => break,
                    };

                    if let Some(square) = board.square(position) {
                        if let Some(piece) = square.get_piece() {
                            if *piece.get_color() != self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position,
                                    modifiers: vec![MoveModifier::Capture],
                                    color: self.color,
                                    piece: self.piece_type,
                                });
                            }

                            break;
                        } else {
                            moves.push(Move {
                                from: self.position,
                                to: position,
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    } else {
                        break;
                    }
                }

                //right
                for i in 1..=8 {
                    let position = match self.position.right(i) {
                        Ok(p) => p,
                        Err(_) => break,
                    };

                    if let Some(square) = board.square(position) {
                        if let Some(piece) = square.get_piece() {
                            if *piece.get_color() != self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position,
                                    modifiers: vec![MoveModifier::Capture],
                                    color: self.color,
                                    piece: self.piece_type,
                                });
                            }

                            break;
                        } else {
                            moves.push(Move {
                                from: self.position,
                                to: position,
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    } else {
                        break;
                    }
                }

                //bottom
                for i in 1..=8 {
                    let position = match self.position.down(i) {
                        Ok(p) => p,
                        Err(_) => break,
                    };

                    if let Some(square) = board.square(position) {
                        if let Some(piece) = square.get_piece() {
                            if *piece.get_color() != self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position,
                                    modifiers: vec![MoveModifier::Capture],
                                    color: self.color,
                                    piece: self.piece_type,
                                });
                            }

                            break;
                        } else {
                            moves.push(Move {
                                from: self.position,
                                to: position,
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    } else {
                        break;
                    }
                }

                //left
                for i in 1..=8 {
                    let position = match self.position.left(i) {
                        Ok(p) => p,
                        Err(_) => break,
                    };

                    if let Some(square) = board.square(position) {
                        if let Some(piece) = square.get_piece() {
                            if *piece.get_color() != self.color {
                                moves.push(Move {
                                    from: self.position,
                                    to: position,
                                    modifiers: vec![MoveModifier::Capture],
                                    color: self.color,
                                    piece: self.piece_type,
                                });
                            }

                            break;
                        } else {
                            moves.push(Move {
                                from: self.position,
                                to: position,
                                modifiers: vec![],
                                color: self.color,
                                piece: self.piece_type,
                            });
                        }
                    } else {
                        break;
                    }
                }

                moves.retain(|m| !m.to.is_oob() && !m.from.is_oob());

                moves
            }
            PieceType::Queen => {
                let mut moves = Vec::new();

                let mut self_as_bishop = self.clone();
                let mut self_as_rook = self.clone();

                self_as_bishop.piece_type = PieceType::Bishop;
                self_as_rook.piece_type = PieceType::Rook;

                moves.append(&mut self_as_bishop.get_moves(board));
                moves.append(&mut self_as_rook.get_moves(board));

                moves
            }
            PieceType::King => {
                let move_nums = vec![
                    self.position.up(1).map(|i| i.right(1)),
                    self.position.up(1).map(|i| i.left(1)),
                    self.position.down(1).map(|i| i.right(1)),
                    self.position.down(1).map(|i| i.left(1)),
                    Ok(self.position.up(1)),
                    Ok(self.position.down(1)),
                    Ok(self.position.right(1)),
                    Ok(self.position.left(1)),
                ];

                let mut moves = Vec::new();

                for position in move_nums.into_iter().flatten().flatten() {
                    let mut capture = false;
                    let mut keep = true;

                    if let Some(Some(piece)) = board.square(position).map(|x| x.get_piece()) {
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

                        moves.push(Move {
                            from: self.position,
                            to: position,
                            modifiers,
                            color: self.color,
                            piece: self.piece_type,
                        });
                    }
                }

                moves.retain(|m| !m.from.is_oob() && !m.to.is_oob());

                moves
            }
        }
    }

    pub fn fen(&self) -> char {
        match self.color {
            Color::White => match self.piece_type {
                PieceType::Pawn => 'P',
                PieceType::Knight => 'N',
                PieceType::Bishop => 'B',
                PieceType::Rook => 'R',
                PieceType::Queen => 'Q',
                PieceType::King => 'K',
            },
            Color::Black => match self.piece_type {
                PieceType::Pawn => 'p',
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook => 'r',
                PieceType::Queen => 'q',
                PieceType::King => 'k',
            },
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self.color {
            Color::White => match self.piece_type {
                PieceType::Bishop => '♗',
                PieceType::King => '♔',
                PieceType::Knight => '♘',
                PieceType::Pawn => '♙',
                PieceType::Queen => '♕',
                PieceType::Rook => '♖',
            },
            Color::Black => match self.piece_type {
                PieceType::Bishop => '♝',
                PieceType::King => '♚',
                PieceType::Knight => '♞',
                PieceType::Pawn => '♟',
                PieceType::Queen => '♛',
                PieceType::Rook => '♜',
            },
        };

        write!(f, "{c}")
    }
}
