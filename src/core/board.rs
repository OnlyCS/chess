use std::ops::{Index, IndexMut};

use anyhow::{bail, ensure, Context, Result};
use rand::{seq::SliceRandom, Rng};

use crate::{
    core::{
        castle::CastleData,
        color::Color,
        file::{File, FileLetter},
        piece::PieceType,
        piece_move::Move,
        piece_move::MoveModifier,
        position::Position,
        square::Square,
    },
    utils::{counter::Counter, traits::ToVec},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameEndedReason {
    FiftyMoveRule,
    Checkmate(Color),
    Stalemate(Color),
}

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
    pub fn new() -> Self {
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

        board.refresh();

        board
    }

    pub fn file(&self, letter: FileLetter) -> Option<&File> {
        self.files.iter().find(|file| file.letter == letter)
    }

    pub fn file_mut(&mut self, letter: FileLetter) -> Option<&mut File> {
        self.files.iter_mut().find(|file| file.letter == letter)
    }

    pub fn square(&self, position: Position) -> Option<&Square> {
        self.file(position.file)?.rank(position.rank)
    }

    pub fn square_mut(&mut self, position: Position) -> Option<&mut Square> {
        self.file_mut(position.file)?.rank_mut(position.rank)
    }

    pub fn squares_mut(&mut self) -> Vec<&mut Square> {
        self.files
            .iter_mut()
            .flat_map(|x| x.get_squares_mut())
            .collect()
    }

    fn force_make_move(&mut self, mv: &Move) -> Result<()> {
        // get piece from "from" square
        let piece_from = self
            .square_mut(mv.from)
            .context("Move \"from\" invalid")?
            .get_piece_owned()
            .context("No piece at \"from\"")?;

        // update ep_target
        if mv.modifiers.contains(&MoveModifier::PawnDoubleMove) {
            match mv.color {
                Color::White => self.ep_target = Some(mv.to.down_loop(1)),
                Color::Black => self.ep_target = Some(mv.to.up_loop(1)),
            }
        } else {
            self.ep_target = None;
        }

        // if ep
        if mv.modifiers.contains(&MoveModifier::EnPassant) {
            match mv.color {
                Color::White => self
                    .square_mut(mv.to.down_loop(1))
                    .context("Failed to get ep pawn to kill")?
                    .clear(),
                Color::Black => self
                    .square_mut(mv.to.down_loop(1))
                    .context("Failed to get ep pawn to kill")?
                    .clear(),
            }
        }

        // update castle data
        if piece_from.get_type() == PieceType::Rook {
            match piece_from.get_color() {
                Color::White => {
                    if piece_from.get_position().file == FileLetter::A {
                        self.white_castle.queenside = false;
                    } else if piece_from.get_position().file == FileLetter::H {
                        self.white_castle.kingside = false;
                    }
                }
                Color::Black => {
                    if piece_from.get_position().file == FileLetter::A {
                        self.black_castle.queenside = false;
                    } else if piece_from.get_position().file == FileLetter::H {
                        self.black_castle.kingside = false;
                    }
                }
            }
        } else if piece_from.get_type() == PieceType::King {
            match piece_from.get_color() {
                Color::White => {
                    self.white_castle.queenside = false;
                    self.white_castle.kingside = false;
                }
                Color::Black => {
                    self.black_castle.queenside = false;
                    self.black_castle.kingside = false;
                }
            }
        }

        // if castle
        if mv.modifiers.contains(&MoveModifier::CastleKingSide) {
            match mv.color {
                Color::White => {
                    let rook_to = mv.from.left_loop(1);
                    let rook_from = mv.from.left_loop(3);

                    let rook_pc_from = self
                        .square_mut(rook_from)
                        .context("Failed to find rook")?
                        .get_piece_owned()
                        .context("Failed to find rook")?;

                    self.square_mut(rook_to)
                        .context("Failed to find rook")?
                        .set_piece(rook_pc_from);

                    self.square_mut(rook_from)
                        .context("Failed to find rook")?
                        .clear();
                }
                Color::Black => {
                    let rook_to = mv.from.right_loop(1);
                    let rook_from = mv.from.right_loop(3);

                    let rook_pc_from = self
                        .square_mut(rook_from)
                        .context("Failed to find rook")?
                        .get_piece_owned()
                        .context("Failed to find rook")?;

                    self.square_mut(rook_to)
                        .context("Failed to find rook")?
                        .set_piece(rook_pc_from);

                    self.square_mut(rook_from)
                        .context("Failed to find rook")?
                        .clear();
                }
            }
        } else if mv.modifiers.contains(&MoveModifier::CastleQueenSide) {
            match mv.color {
                Color::White => {
                    let rook_to = mv.from.right_loop(1);
                    let rook_from = mv.from.right_loop(4);

                    let rook_pc_from = self
                        .square_mut(rook_from)
                        .context("Failed to find rook")?
                        .get_piece_owned()
                        .context("Failed to find rook")?;

                    self.square_mut(rook_to)
                        .context("Failed to find rook")?
                        .set_piece(rook_pc_from);

                    self.square_mut(rook_from)
                        .context("Failed to find rook")?
                        .clear();
                }
                Color::Black => {
                    let rook_to = mv.from.left_loop(1);
                    let rook_from = mv.from.left_loop(4);

                    let rook_pc_from = self
                        .square_mut(rook_from)
                        .context("Failed to find rook")?
                        .get_piece_owned()
                        .context("Failed to find rook")?;

                    self.square_mut(rook_to)
                        .context("Failed to find rook")?
                        .set_piece(rook_pc_from);

                    self.square_mut(rook_from)
                        .context("Failed to find rook")?
                        .clear();
                }
            }
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
        self.square_mut(mv.to)
            .context("Move \"to\" invalid")?
            .set_piece(piece_from);

        // update turn
        self.turn = self.turn.opposite();

        Ok(())
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<()> {
        // verify move is in move_cache
        if !self.move_cache.contains(mv) {
            bail!("Move not in move_cache");
        }

        self.force_make_move(mv)?;

        self.refresh();

        Ok(())
    }

    fn moves_unchecked_regen(&self) -> Vec<Move> {
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
            let pawnpos: Position =
                if let Some(Some(pc)) = self.square(target.up_loop(1)).map(|x| x.get_piece()) {
                    if pc.get_type() == PieceType::Pawn {
                        *pc.get_position()
                    } else {
                        target.down_loop(1)
                    }
                } else {
                    target.down_loop(1)
                };

            if let Ok(piece_right) = pawnpos.clone().right(1) {
                if let Some(Some(piece)) = self.square(piece_right).map(|x| x.get_piece()) {
                    if *piece.get_color() == self.turn && piece.get_type() == PieceType::Pawn {
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

            if let Ok(piece_left) = pawnpos.clone().left(1) {
                if let Some(Some(piece)) = self.square(piece_left).map(|x| x.get_piece()) {
                    if *piece.get_color() == self.turn && piece.get_type() == PieceType::Pawn {
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

        moves
    }

    pub fn refresh(&mut self) {
        let mut moves = self.moves_unchecked_regen();

        self.filter_check(&mut moves);

        self.move_cache = moves;
    }

    pub fn get_moves(&self) -> Vec<Move> {
        self.move_cache.clone()
    }

    pub fn is_check(&self) -> Option<Color> {
        let mut working_board = self.clone();

        let moves_for_white = {
            working_board.turn = Color::White;
            working_board.moves_unchecked_regen()
        };

        let moves_for_black = {
            working_board.turn = Color::Black;
            working_board.moves_unchecked_regen()
        };

        // fixes a bug where the black king is in check but a valid move is to put the white king in check
        let check_first = self.turn.opposite() == Color::White;

        let (moves1, moves2) = if check_first {
            (&moves_for_white, &moves_for_black)
        } else {
            (&moves_for_black, &moves_for_white)
        };

        #[allow(clippy::unwrap_used)]
        for mv in moves1 {
            if mv.modifiers.contains(&MoveModifier::Capture) {
                if let Some(pc) = self.square(mv.to).unwrap().get_piece() {
                    if pc.get_type() == PieceType::King {
                        return Some(if check_first {
                            Color::Black
                        } else {
                            Color::White
                        });
                    }
                }
            }
        }

        #[allow(clippy::unwrap_used)]
        for mv in moves2 {
            if mv.modifiers.contains(&MoveModifier::Capture) {
                if let Some(pc) = self.square(mv.to).unwrap().get_piece() {
                    if pc.get_type() == PieceType::King {
                        return Some(if check_first {
                            Color::White
                        } else {
                            Color::Black
                        });
                    }
                }
            }
        }

        None
    }

    pub fn filter_check(&self, moves: &mut Vec<Move>) {
        moves.retain(|mv| {
            let mut working_board = self.clone();

            working_board.turn = mv.color;

            match working_board.force_make_move(mv) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {:?} {:?}", mv, e);
                    return false;
                }
            }

            if let Some(c) = working_board.is_check() {
                c != mv.color
            } else {
                true
            }
        })
    }

    pub fn checkmate(&self) -> Option<Color> {
        let mut working_board = self.clone();
        let original_turn = working_board.turn;

        let moves_white = match working_board.turn {
            Color::White => working_board.get_moves(),
            Color::Black => {
                working_board.turn = Color::Black;
                working_board.refresh();
                working_board.get_moves()
            }
        };

        let moves_black = match working_board.turn {
            Color::White => {
                working_board.turn = Color::White;
                working_board.refresh();
                working_board.get_moves()
            }
            Color::Black => working_board.get_moves(),
        };

        working_board.turn = original_turn;
        working_board.refresh();

        let can_be_mate = if moves_white.is_empty() {
            Some(Color::White)
        } else if moves_black.is_empty() {
            Some(Color::Black)
        } else {
            None
        };

        // dont detect stalemate
        if let Some(c) = can_be_mate {
            if let Some(check_col) = self.is_check() {
                if c == check_col {
                    return can_be_mate;
                }
            }
        }

        None
    }

    pub fn stalemate(&self) -> Option<Color> {
        let mut working_board = self.clone();
        let original_turn = working_board.turn;

        let moves_white = match working_board.turn {
            Color::White => working_board.get_moves(),
            Color::Black => {
                working_board.turn = Color::Black;
                working_board.refresh();
                working_board.get_moves()
            }
        };

        let moves_black = match working_board.turn {
            Color::White => {
                working_board.turn = Color::White;
                working_board.refresh();
                working_board.get_moves()
            }
            Color::Black => working_board.get_moves(),
        };

        working_board.turn = original_turn;
        working_board.refresh();

        let can_be_mate = if moves_white.is_empty() {
            Some(Color::White)
        } else if moves_black.is_empty() {
            Some(Color::Black)
        } else {
            None
        };

        // dont detect stalemate
        if let Some(c) = can_be_mate {
            if let Some(check_col) = self.is_check() {
                if c != check_col {
                    return can_be_mate;
                }
            }
        }

        None
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

    pub fn turn(&self) -> &Color {
        &self.turn
    }

    pub fn game_has_ended(&self) -> Option<GameEndedReason> {
        if let Some(c) = self.checkmate() {
            return Some(GameEndedReason::Checkmate(c));
        }

        if let Some(c) = self.stalemate() {
            return Some(GameEndedReason::Stalemate(c));
        }

        if self.fifty_ctr.get() >= 50 {
            return Some(GameEndedReason::FiftyMoveRule);
        }

        None
    }

    pub fn promote(&mut self, color: Color, to: PieceType) -> Result<()> {
        let mut success = false;

        match color {
            Color::White => {
                // search for pawns on the last rank
                for file in self.files.iter_mut() {
                    for square in file.to_vec_mut() {
                        if let Some(piece) = square.get_piece_mut() {
                            if piece.get_type() == PieceType::Pawn
                                && *piece.get_color() == Color::White
                                && piece.get_position().rank == 8
                            {
                                piece.piece_type = to;
                                success = true;
                                break;
                            }
                        }
                    }
                }
            }
            Color::Black => {
                // search for pawns on the last rank
                for file in self.files.iter_mut() {
                    for square in file.to_vec_mut() {
                        if let Some(piece) = square.get_piece_mut() {
                            if piece.get_type() == PieceType::Pawn
                                && *piece.get_color() == Color::Black
                                && piece.get_position().rank == 1
                            {
                                piece.piece_type = to;
                                success = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        ensure!(success, "No pawn to promote");

        Ok(())
    }

    pub fn random() -> Result<Self> {
        let mut board = Self::default();

        // for every board.square
        let squares: Vec<&mut Square> = board.to_vec_mut();
        let mut pcs = vec![];

        for square in squares {
            if let Some(pc) = square.get_piece_owned() {
                pcs.push(pc);
            }
        }

        let mut kings = pcs
            .drain_filter(|pc| pc.get_type() == PieceType::King)
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        let num_pcs = rng.gen_range(8..=20);

        // use rng to gen two numbers 1-8 (rank, file)

        let king0pos = (rng.gen_range(1..=8), rng.gen_range(1..=8));
        let king1pos;

        loop {
            let k1testpos = (rng.gen_range(1..=8), rng.gen_range(1..=8));

            if k1testpos != king0pos {
                king1pos = k1testpos;
                break;
            }
        }

        kings
            .index_mut(0)
            .set_position(Position::new(FileLetter::from(king0pos.1), king0pos.0));

        kings
            .index_mut(1)
            .set_position(Position::new(FileLetter::from(king1pos.1), king1pos.0));

        board
            .square_mut(Position::new(FileLetter::from(king0pos.1), king0pos.0))
            .context("Failed to set king0pos")?
            .set_piece(kings.index(0).clone());

        board
            .square_mut(Position::new(FileLetter::from(king1pos.1), king1pos.0))
            .context("Failed to set king1pos")?
            .set_piece(kings.index(1).clone());

        for _ in 0..num_pcs {
            let pos;

            loop {
                let testpos =
                    Position::new(FileLetter::from(rng.gen_range(1..=8)), rng.gen_range(1..=8));

                if board
                    .square(testpos)
                    .map(|s| s.get_piece().is_none())
                    .unwrap_or(false)
                {
                    pos = testpos;
                    break;
                }
            }

            let pc = pcs.choose(&mut rng);

            if let Some(pc) = pc {
                let mut pc = pc.clone();
                pc.set_position(pos);
                board
                    .square_mut(pos)
                    .context("Failed to get position")?
                    .set_piece(pc);
            }
        }

        Ok(board)
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

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
