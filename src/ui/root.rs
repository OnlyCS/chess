use crossterm::{
    event::{KeyEvent, MouseEvent},
    terminal::size,
};

use intuitive::{
    components::{experimental::modal::Modal, *},
    event::handler::Propagate,
    state::use_state,
    *,
};

use crate::{
    game::chess::Chess,
    parts::position::Position,
    pieces::{bishop::Bishop, knight::Knight, queen::Queen, rook::Rook},
    types::{
        color::Color,
        piece_type::PieceType,
        r#move::{Move, MoveFilter, MoveModifier},
    },
};

use super::{
    board::Board,
    data::{SelectData, UIFileData},
    promote::Promote,
    selection::{Selection, SelectionMode},
};

#[component(Root)]
pub fn render() -> element::Any {
    let chess = use_state(Chess::default);
    let select_mode = use_state(|| SelectionMode::SelectPiece);
    let error_message = use_state(String::new);
    let check = use_state(|| false);
    let checkmate = use_state(|| false);
    let promotion = use_state(|| false);
    let selection = use_state(|| Selection {
        hover: Position::default(),
        selected: None,
        avaliable: vec![],
    });

    let helper_text: String;
    let board_data = chess
        .get()
        .get_board()
        .get_files()
        .iter()
        .map(|f| f.get_squares())
        .map(|p| {
            p.iter()
                .map(|s| SelectData {
                    selection: selection.get().has(s.get_position()),
                    piece: s
                        .get_piece()
                        .map(|p| p.to_string())
                        .unwrap_or(" ".to_string()),
                })
                .rev()
                .collect::<Vec<SelectData>>()
        })
        .map(|f| UIFileData::create_from(f.to_vec()))
        .collect::<Vec<UIFileData>>();

    // handle key events
    let key_hander = {
        let promotion = promotion.clone();
        let checkmate = checkmate.clone();
        let select_mode = select_mode.clone();
        let error_message = error_message.clone();
        let check = check.clone();

        move |event: KeyEvent| {
            use intuitive::event::KeyCode::*;

            let is_promote = promotion.get();
            let is_checkmate = checkmate.get();

            if is_checkmate {
                event::quit();
            }

            if is_promote {
                let promotion = promotion.clone();
                let error_message = error_message.clone();
                let game = chess.get();

                let mut pawns = game
                    .get_board()
                    .get_pieces()
                    .iter()
                    .filter(|p| p.get_type() == PieceType::Pawn)
                    .filter(|p| match *p.get_color() {
                        Color::White => p.get_position().rank == 8,
                        Color::Black => p.get_position().rank == 1,
                    })
                    .copied()
                    .collect::<Vec<_>>();

                if pawns.len() != 1 {
                    error_message.set("Something went wrong".to_string());
                }

                chess.mutate(|game| {
                    if let Some(p) = pawns.pop() {
                        if let Some(s) = game.get_board_mut().square_mut(p.get_position()) {
                            match event.code {
                                Char('b') => s.set_piece(Box::new(Bishop::new(
                                    *p.get_color(),
                                    p.get_position().clone(),
                                ))),
                                Char('k') => s.set_piece(Box::new(Knight::new(
                                    *p.get_color(),
                                    p.get_position().clone(),
                                ))),
                                Char('r') => s.set_piece(Box::new(Rook::new(
                                    *p.get_color(),
                                    p.get_position().clone(),
                                ))),
                                Char('q') => s.set_piece(Box::new(Queen::new(
                                    *p.get_color(),
                                    p.get_position().clone(),
                                ))),
                                _ => {
                                    error_message.set("Invalid promotion".to_string());
                                }
                            }
                        }
                    }
                });

                promotion.set(false);
                return Propagate::Next;
            }

            match event.code {
                Char('q') => event::quit(),
                Char('w') | Up => {
                    selection.mutate(|s| s.hover = s.hover.up(1).unwrap_or(s.hover.clone()))
                }
                Char('a') | Left => {
                    selection.mutate(|s| s.hover = s.hover.left(1).unwrap_or(s.hover.clone()))
                }
                Char('s') | Down => {
                    selection.mutate(|s| s.hover = s.hover.down(1).unwrap_or(s.hover.clone()))
                }
                Char('d') | Right => {
                    selection.mutate(|s| s.hover = s.hover.right(1).unwrap_or(s.hover.clone()))
                }
                Enter => {
                    match select_mode.get() {
                        SelectionMode::SelectPiece => {
                            let game = chess.get();
                            let board = game.get_board();

                            // check for mate
                            if board.is_mate(*game.get_turn()) {
                                checkmate.set(true);
                                return Propagate::Next;
                            }

                            // check to make sure there is a piece at the selected square
                            let selected_piece = match board
                                .square(&selection.get().hover)
                                .and_then(|s| s.get_piece())
                            {
                                Some(p) => p,
                                _ => {
                                    error_message.set("No piece at these coordinates".to_string());
                                    return Propagate::Next;
                                }
                            };

                            // check to make sure it is the correct color's turn
                            if *selected_piece.get_color() != *game.get_turn() {
                                error_message.set("Not your turn".to_string());
                                return Propagate::Next;
                            }

                            // get the moves for the selected piece
                            let mut moves = selected_piece.get_moves(board);
                            moves.filter_king_check(game.get_board(), *game.get_turn());

                            selection.mutate(|s| {
                                s.selected = Some(s.hover.clone());
                                s.avaliable = moves;
                            });

                            select_mode.set(SelectionMode::SelectMove);
                            error_message.set(String::new());

                            if selection.get().avaliable.is_empty() {
                                error_message.set("\nWarning: This piece has no moves. \nPress Esc to go back into piece selection mode".to_string());
                            }
                        }
                        SelectionMode::SelectMove => {
                            chess.mutate(|game| {
                                let mut moves = selection.get().avaliable;
                                let move_to = selection.get().hover;

                                // check to make sure the move is valid
                                if !moves
                                    .iter()
                                    .map(|m| m.clone().to)
                                    .collect::<Vec<Position>>()
                                    .contains(&move_to)
                                {
                                    error_message.set("Invalid move".to_string());
                                    return;
                                }

                                // grab the move
                                moves.retain(|m| m.to == move_to);
                                let mv = match moves.pop() {
                                    Some(m) => m,
                                    None => {
                                        error_message.set("Oops, something went wrong".to_string());
                                        return;
                                    }
                                };

                                // en passant
                                if mv.modifiers.contains(&MoveModifier::EnPassant) {
                                    let mut en_passant_square = mv.to.clone();
                                    en_passant_square.rank = match *game.get_turn() {
                                        Color::White => en_passant_square.rank - 1,
                                        Color::Black => en_passant_square.rank + 1,
                                    };
                                    match game
                                        .get_board_mut()
                                        .square_mut(&en_passant_square)
                                        .map(|s| s.clear())
                                    {
                                        Some(_) => (),
                                        None => return,
                                    }
                                }

                                // castling queenside
                                if mv.modifiers.contains(&MoveModifier::CastleQueenSide) {
                                    let rook_from = mv.to.clone().left(2).expect("Unreachable");
                                    let rook_to = mv.to.clone().right(1).expect("Unreachable");

                                    match game.get_board_mut().make_move(&Move::new(
                                        rook_from,
                                        rook_to,
                                        vec![],
                                    )) {
                                        Ok(_) => (),
                                        Err(e) => {
                                            error_message.set(format!("Movement failed: {e}"));
                                            return;
                                        }
                                    }
                                }

                                // castling kingside
                                if mv.modifiers.contains(&MoveModifier::CastleKingSide) {
                                    let rook_from = mv.to.clone().right(1).expect("Unreachable");
                                    let rook_to = mv.to.clone().left(1).expect("Unreachable");

                                    match game.make_move(&Move::new(rook_from, rook_to, vec![])) {
                                        Ok(_) => (),
                                        Err(e) => {
                                            error_message.set(format!("Movement failed: {e}"));
                                            return;
                                        }
                                    }
                                }

                                // move piece
                                let move_result = game.make_move(&mv);
                                if let Err(e) = move_result {
                                    error_message.set(format!("Movement failed: {e}"));
                                    return;
                                }

                                // reset
                                select_mode.set(SelectionMode::SelectPiece);
                                error_message.set(String::new());
                                selection.mutate(|s| {
                                    s.selected = None;
                                    s.avaliable = vec![];
                                });

                                // promotion
                                if mv.modifiers.contains(&MoveModifier::PromotionUnknown) {
                                    promotion.set(true);
                                }

                                // check
                                if game.get_board().is_check(Color::Black)
                                    || game.get_board().is_check(Color::White)
                                {
                                    check.set(true);
                                } else {
                                    check.set(false);
                                }
                            });
                        }
                    }
                }
                Esc => match select_mode.get() {
                    SelectionMode::SelectPiece => (),
                    SelectionMode::SelectMove => {
                        select_mode.set(SelectionMode::SelectPiece);
                        error_message.set(String::new());
                        selection.mutate(|s| {
                            s.selected = None;
                            s.avaliable = vec![];
                        });
                    }
                },
                _ => (),
            }

            Propagate::Next
        }
    };

    // remove mouse handler
    let mouse_handler = move |_: MouseEvent| Propagate::Stop;

    // flexing
    let (term_width, term_height) = size().expect("Error message");

    let min_term_width = 151;
    let min_term_height = 34;
    let large_enough = term_width >= min_term_width && term_height >= min_term_height;

    let flex = if large_enough {
        helper_text = match select_mode.get() {
			SelectionMode::SelectPiece => "WASD/Arrow Keys to move selection\nEnter to select a piece\nq to quit".to_string(),
			SelectionMode::SelectMove => "WASD/Arrow Keys to move selection\nEnter to move the piece\nEsc to select a different piece\nq to quit".to_string(),
		};

        (50, term_width - 50, 26, term_height - 26)
    } else {
        helper_text = format!("Increase terminal size\nCurrent: {term_width}x{term_height}\nRequired: {min_term_width}x{min_term_height}");
        (0, 1, 1, 0)
    };

    if error_message.get() != String::new()
        && !error_message.get().trim().starts_with("ERROR:")
        && !error_message
            .get()
            .trim()
            .to_lowercase()
            .starts_with("warning:")
    {
        error_message.set(format!("\nERROR: {}", error_message.get()));
    }

    render! {
        VStack(on_key: key_hander, on_mouse: mouse_handler, flex: [flex.2, flex.3]) {
            HStack(flex: [flex.0, flex.1]) {
                Section(title: "Board") {
                    Board(board_data: board_data)
                }

                Section(title: "Instructions") {
                    Centered() {
                        VStack() {
                            Text(text: helper_text)
                            Text(text: format!("{}{}", error_message.get(), if checkmate.get() { "\nCHECKMATE" } else if check.get() { "\nCHECK" } else { "" }))
                        }
                    }
                }
            }

            Modal() {
                Promote(shown: promotion.get())
            }
        }
    }
}
