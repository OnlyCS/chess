use anyhow::{bail, Context, Result};

use crate::{
    core::{
        castle::CastleData, color::Color, file::File, file::FileLetter, piece::PieceType,
        piece_move::Move, piece_move::MoveModifier, position::Position, square::Square,
    },
    utils::{counter::Counter, to_vec::ToVec},
};

#[derive(Debug, Clone)]
pub struct Board {
    files: Vec<File>,
    ep_target: Option<Position>,
    fifty_ctr: Counter<u8>,
    move_ctr: Counter<u8>,
    move_cache: Vec<Move>,
    turn: Color,
    white_castle: CastleData,
    black_castle: CastleData,
}

impl Board {
    pub fn new() -> Result<Self> {
        let mut board = Self {
            files: Vec::new(),
            ep_target: None,
            fifty_ctr: Counter::new(0),
            move_ctr: Counter::new(1),
            move_cache: vec![],
            turn: Color::White,
            white_castle: CastleData {
                queenside: true,
                kingside: true,
                color: Color::White,
            },
            black_castle: CastleData {
                queenside: true,
                kingside: true,
                color: Color::Black,
            },
        };

        for letter in FileLetter::vec_all() {
            board.files.push(File::new(letter));
        }

        board.refresh()?;

        Ok(board)
    }

    pub fn file(&self, letter: FileLetter) -> Option<&File> {
        self.files.iter().find(|file| file.letter == letter)
    }

    pub fn file_mut(&mut self, letter: FileLetter) -> Option<&mut File> {
        self.files.iter_mut().find(|file| file.letter == letter)
    }

    pub fn square(&self, position: &Position) -> Option<&Square> {
        self.file(position.file)?.rank(position.rank)
    }

    pub fn square_mut(&mut self, position: &Position) -> Option<&mut Square> {
        self.file_mut(position.file)?.rank_mut(position.rank)
    }

    pub fn squares_mut(&mut self) -> Vec<&mut Square> {
        self.files
            .iter_mut()
            .flat_map(|x| x.get_squares_mut())
            .collect()
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<()> {
        // verify move is in move_cache
        if !self.move_cache.contains(mv) {
            bail!("Move not in move_cache");
        }

        // get piece from "from" square
        let piece_from = self
            .square_mut(&mv.from)
            .context("Move \"from\" invalid")?
            .get_piece_owned()
            .context("No piece at \"from\"")?;

        // update ep_target
        if mv.modifiers.contains(&MoveModifier::PawnDoubleMove) {
            self.ep_target = Some(mv.to.down_loop(1));
        } else {
            self.ep_target = None;
        }

        // update fifty_ctr
        if piece_from.get_type() == PieceType::Pawn || mv.modifiers.contains(&MoveModifier::Capture)
        {
            self.fifty_ctr.reset();
        } else {
            self.fifty_ctr.inc();
        }

        // update move_ctr
        if self.turn == Color::Black {
            self.move_ctr.inc();
        }

        // move piece
        self.square_mut(&mv.to)
            .context("Move \"to\" invalid")?
            .set_piece(piece_from);

        // update turn
        self.turn = self.turn.get_opposite();

        self.refresh()?;

        Ok(())
    }

    pub fn refresh(&mut self) -> Result<()> {
        let mut moves = Vec::new();

        for square in <Board as ToVec<Square>>::to_vec(self) {
            if let Some(piece) = square.get_piece() {
                if *piece.get_color() == self.turn {
                    moves.append(&mut piece.get_moves(self));
                }
            }
        }

        // add en passant moves
        if let Some(target) = self.ep_target {
            if let Ok(piece_right) = target.clone().up_loop(1).right(1) {
                if let Some(Some(piece)) = self.square(&piece_right).map(|x| x.get_piece()) {
                    if *piece.get_color() == self.turn.get_opposite()
                        && piece.get_type() == PieceType::Pawn
                    {
                        moves.push(Move {
                            from: piece_right,
                            to: target,
                            modifiers: vec![MoveModifier::EnPassant],
                            piece: PieceType::Pawn,
                            color: self.turn,
                        });
                    }
                }
            }

            if let Ok(piece_left) = target.clone().up_loop(1).left(1) {
                if let Some(Some(piece)) = self.square(&piece_left).map(|x| x.get_piece()) {
                    if *piece.get_color() == self.turn.get_opposite()
                        && piece.get_type() == PieceType::Pawn
                    {
                        moves.push(Move {
                            from: piece_left,
                            to: target,
                            modifiers: vec![MoveModifier::EnPassant],
                            piece: PieceType::Pawn,
                            color: self.turn,
                        });
                    }
                }
            }
        }

        self.filter_check(&mut moves)?;

        self.move_cache = moves;

        Ok(())
    }

    pub fn get_moves(&self) -> Vec<Move> {
        self.move_cache.clone()
    }

    pub fn is_check(&self) -> Option<Color> {
        let mut working_board = self.clone();
        let original_turn = working_board.turn;

        let moves_white = match working_board.turn {
            Color::White => working_board.get_moves(),
            Color::Black => {
                working_board.turn = Color::Black;
                working_board.refresh().ok()?;
                working_board.get_moves()
            }
        };

        let moves_black = match working_board.turn {
            Color::White => {
                working_board.turn = Color::White;
                working_board.refresh().ok()?;
                working_board.get_moves()
            }
            Color::Black => working_board.get_moves(),
        };

        working_board.turn = original_turn;
        working_board.refresh().ok()?;

        for mv in moves_white {
            let mut sub_working_board = working_board.clone();
            sub_working_board.make_move(&mv).ok()?;

            // count kings in board
            let kings = sub_working_board
                .to_vec()
                .iter()
                .filter_map(|s: &&Square| s.get_piece())
                .filter(|p| p.get_type() == PieceType::King)
                .count();

            if kings != 2 {
                return Some(Color::White);
            }
        }

        for mv in moves_black {
            let mut sub_working_board = working_board.clone();
            sub_working_board.make_move(&mv).ok()?;

            // count kings in board
            let kings = sub_working_board
                .to_vec()
                .iter()
                .filter_map(|s: &&Square| s.get_piece())
                .filter(|p| p.get_type() == PieceType::King)
                .count();

            if kings != 2 {
                return Some(Color::Black);
            }
        }

        None
    }

    pub fn filter_check(&self, moves: &mut Vec<Move>) -> Result<()> {
        *moves = moves
            .iter()
            .filter(|mv| {
                let mut working_board = self.clone();

                working_board.turn = mv.color;

                match working_board.make_move(mv) {
                    Ok(_) => {}
                    Err(_) => return false,
                };

                match working_board.refresh() {
                    Ok(_) => {}
                    Err(_) => return false,
                }

                working_board.turn = working_board.turn.get_opposite();

                if let Some(c) = working_board.is_check() {
                    c != mv.color
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        Ok(())
    }

    pub fn checkmate(&self) -> Option<Color> {
        let mut working_board = self.clone();
        let original_turn = working_board.turn;

        let moves_white = match working_board.turn {
            Color::White => working_board.get_moves(),
            Color::Black => {
                working_board.turn = Color::Black;
                working_board.refresh().ok()?;
                working_board.get_moves()
            }
        };

        let moves_black = match working_board.turn {
            Color::White => {
                working_board.turn = Color::White;
                working_board.refresh().ok()?;
                working_board.get_moves()
            }
            Color::Black => working_board.get_moves(),
        };

        working_board.turn = original_turn;
        working_board.refresh().ok()?;

        if moves_white.is_empty() {
            Some(Color::White)
        } else if moves_black.is_empty() {
            Some(Color::Black)
        } else {
            None
        }
    }

    pub fn fen(&self) -> String {
        let mut ranks = vec![Vec::with_capacity(8); 8];
        let squares: Vec<&Square> = self.to_vec();

        for square in squares {
            let rank = square.get_position().rank as usize;
            let file = square.get_position().file as usize;

            ranks[rank][file] = square.fen();
        }

        let mut fen = String::new();

        for rank in ranks {
            let mut empty = 0;

            for square in rank {
                if square == '1' {
                    empty += 1;
                } else {
                    if empty > 0 {
                        fen.push_str(&empty.to_string());
                        empty = 0;
                    }

                    fen.push(square);
                }
            }

            if empty > 0 {
                fen.push_str(&empty.to_string());
            }

            fen.push('/');
        }

        let castle_str = {
            let mut castle_str = String::new();

            if self.white_castle.fen() == "-" && self.black_castle.fen() == "-" {
                castle_str.push('-');
            } else {
                if self.white_castle.fen() != "-" {
                    castle_str.push_str(&self.white_castle.fen());
                }

                if self.black_castle.fen() != "-" {
                    castle_str.push_str(&self.black_castle.fen());
                }
            }

            castle_str
        };

        let en_passant_str = {
            let mut en_passant_str = String::new();

            if let Some(ep_target) = self.ep_target {
                en_passant_str.push_str(&ep_target.fen());
            } else {
                en_passant_str.push('-');
            }

            en_passant_str
        };

        fen.push_str(&format!(
            " {} {} {} {} {}",
            self.turn.fen(),
            castle_str,
            en_passant_str,
            self.move_ctr.get(),
            self.fifty_ctr.get()
        ));

        fen
    }

    pub fn turn(&self) -> Color {
        self.turn
    }
}

impl ToVec<Square> for Board {
    fn to_vec(&self) -> Vec<&Square> {
        self.files.iter().flat_map(|f| f.to_vec()).collect()
    }

    fn to_vec_mut(&mut self) -> Vec<&mut Square> {
        self.files.iter_mut().flat_map(|f| f.to_vec_mut()).collect()
    }
}

impl ToVec<File> for Board {
    fn to_vec(&self) -> Vec<&File> {
        self.files.iter().collect()
    }

    fn to_vec_mut(&mut self) -> Vec<&mut File> {
        self.files.iter_mut().collect()
    }
}
