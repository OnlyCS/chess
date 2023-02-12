use crossterm::{event::MouseEvent, terminal::size};
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
        r#move::{MoveFilter, MoveModifier},
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

    let key_hander = {
        let error_message = error_message.clone();
        let select_mode = select_mode.clone();
        let check = check.clone();
        let promotion = promotion.clone();
        let checkmate = checkmate.clone();

        move |event| {
            if !promotion.get() && !checkmate.get() {
                match select_mode.get() {
                    SelectionMode::SelectPiece => {
                        use intuitive::event::{
                            KeyCode::{self, *},
                            KeyEvent,
                        };

                        match event {
                            KeyEvent {
                                code: Char('q'), ..
                            } => event::quit(),
                            KeyEvent {
                                code: Char('w'), ..
                            }
                            | KeyEvent { code: Up, .. } => selection
                                .mutate(|s| s.hover = s.hover.up(1).unwrap_or(s.hover.clone())),
                            KeyEvent {
                                code: Char('a'), ..
                            }
                            | KeyEvent { code: Left, .. } => selection
                                .mutate(|s| s.hover = s.hover.left(1).unwrap_or(s.hover.clone())),
                            KeyEvent {
                                code: Char('s'), ..
                            }
                            | KeyEvent { code: Down, .. } => selection
                                .mutate(|s| s.hover = s.hover.down(1).unwrap_or(s.hover.clone())),
                            KeyEvent {
                                code: Char('d'), ..
                            }
                            | KeyEvent { code: Right, .. } => selection
                                .mutate(|s| s.hover = s.hover.right(1).unwrap_or(s.hover.clone())),
                            KeyEvent {
                                code: KeyCode::Enter,
                                ..
                            } => {
                                let game = chess.get();

                                let selected_piece = match game
                                    .get_board()
                                    .square(&selection.get().hover)
                                    .map(|s| s.get_piece())
                                {
                                    Some(Some(p)) => p,
                                    _ => {
                                        error_message
                                            .set("No piece at these coordinates".to_string());
                                        return Propagate::Next;
                                    }
                                };

                                if *selected_piece.get_color() != *game.get_turn() {
                                    error_message.set("Not your turn".to_string());
                                    return Propagate::Next;
                                }

                                let mut moves = selected_piece.get_moves(chess.get().get_board());
                                moves.filter_king_check(game.get_board(), *game.get_turn());

                                let mut cmtest = game.get_moves();
                                cmtest.filter_king_check(game.get_board(), *game.get_turn());

                                if cmtest.is_empty() {
                                    checkmate.set(true);
                                    return Propagate::Next;
                                }

                                selection.mutate(|s| {
                                    s.selected = Some(s.hover.clone());
                                    s.avaliable = moves.iter().map(|m| m.to.clone()).collect();
                                });

                                select_mode.set(SelectionMode::SelectMove);
                                error_message.set(String::new());
                                if selection.get().avaliable.is_empty() {
                                    error_message.set("\nWarning: This piece has no moves. \nPress Esc to go back into piece selection mode".to_string());
                                }
                            }

                            _ => (),
                        }
                    }
                    SelectionMode::SelectMove => {
                        use intuitive::event::{KeyCode::*, KeyEvent};

                        match event {
                            KeyEvent {
                                code: Char('q'), ..
                            } => event::quit(),
                            KeyEvent {
                                code: Char('w'), ..
                            }
                            | KeyEvent { code: Up, .. } => selection
                                .mutate(|s| s.hover = s.hover.up(1).unwrap_or(s.hover.clone())),
                            KeyEvent {
                                code: Char('a'), ..
                            }
                            | KeyEvent { code: Left, .. } => selection
                                .mutate(|s| s.hover = s.hover.left(1).unwrap_or(s.hover.clone())),
                            KeyEvent {
                                code: Char('s'), ..
                            }
                            | KeyEvent { code: Down, .. } => selection
                                .mutate(|s| s.hover = s.hover.down(1).unwrap_or(s.hover.clone())),
                            KeyEvent {
                                code: Char('d'), ..
                            }
                            | KeyEvent { code: Right, .. } => selection
                                .mutate(|s| s.hover = s.hover.right(1).unwrap_or(s.hover.clone())),
                            KeyEvent { code: Esc, .. } => {
                                select_mode.set(SelectionMode::SelectPiece);

                                selection.mutate(|s| {
                                    s.selected = None;
                                    s.avaliable = vec![];
                                });
                                error_message.set(String::new());
                            }
                            KeyEvent { code: Enter, .. } => {
                                chess.mutate(|game| {
                                    let move_to = selection.get().hover;
                                    let move_from = match selection.get().selected {
                                        Some(s) => s,
                                        None => {
                                            error_message.set("No piece selected".to_string());
                                            return;
                                        }
                                    };
                                    let available_movetos = selection.get().avaliable;

                                    let mut moves =
                                        match game.get_board().square(&move_from).map(|s| {
                                            s.get_piece().map(|p| p.get_moves(game.get_board()))
                                        }) {
                                            Some(Some(m)) => m,
                                            _ => {
                                                error_message.set(
                                                    "No piece at these coordinates".to_string(),
                                                );
                                                return;
                                            }
                                        };

                                    if !available_movetos.contains(&move_to) {
                                        error_message
                                            .set("Invalid move: Not in existing list".to_string());
                                        return;
                                    }

                                    moves.retain(|m| m.to == move_to);

                                    let mv = match moves.pop() {
                                        Some(m) => m,
                                        None => {
                                            error_message
                                                .set("Invalid move: No moves left".to_string());
                                            return;
                                        }
                                    };

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

                                    let move_result = game.make_move(&mv);

                                    if let Err(e) = move_result {
                                        error_message.set(format!("Movement failed: {e}"));
                                        return;
                                    }

                                    select_mode.set(SelectionMode::SelectPiece);

                                    selection.mutate(|s| {
                                        s.selected = None;
                                        s.avaliable = vec![];
                                    });
                                    error_message.set(String::new());

                                    if mv.modifiers.contains(&MoveModifier::Promotion) {
                                        promotion.set(true);
                                    }
                                });

                                if chess.get().get_board().is_check(Color::Black)
                                    || chess.get().get_board().is_check(Color::White)
                                {
                                    check.set(true);
                                } else {
                                    check.set(false);
                                }
                            }
                            _ => (),
                        }
                    }
                };
            } else if promotion.get() && !checkmate.get() {
                use intuitive::event::{KeyCode::*, KeyEvent};

                let game = chess.get();

                let pawns = game
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

                match event {
                    KeyEvent {
                        code: Char('b'), ..
                    } => chess.mutate(|game| {
                        if let Some(s) = game.get_board_mut().square_mut(pawns[0].get_position()) {
                            s.set_piece(Box::new(Bishop::new(
                                *pawns[0].get_color(),
                                pawns[0].get_position().clone(),
                            )))
                        } else {
                            error_message.set("Something went wrong".to_string());
                        }
                    }),
                    KeyEvent {
                        code: Char('k'), ..
                    } => chess.mutate(|game| {
                        if let Some(s) = game.get_board_mut().square_mut(pawns[0].get_position()) {
                            s.set_piece(Box::new(Knight::new(
                                *pawns[0].get_color(),
                                pawns[0].get_position().clone(),
                            )));
                        } else {
                            error_message.set("Something went wrong".to_string());
                        }
                    }),
                    KeyEvent {
                        code: Char('r'), ..
                    } => chess.mutate(|game| {
                        if let Some(s) = game.get_board_mut().square_mut(pawns[0].get_position()) {
                            s.set_piece(Box::new(Rook::new(
                                *pawns[0].get_color(),
                                pawns[0].get_position().clone(),
                            )));
                        } else {
                            error_message.set("Something went wrong".to_string());
                        }
                    }),
                    KeyEvent {
                        code: Char('q'), ..
                    } => chess.mutate(|game| {
                        if let Some(s) = game.get_board_mut().square_mut(pawns[0].get_position()) {
                            s.set_piece(Box::new(Queen::new(
                                *pawns[0].get_color(),
                                pawns[0].get_position().clone(),
                            )));
                        } else {
                            error_message.set("Something went wrong".to_string());
                        }
                    }),
                    _ => (),
                }

                promotion.set(false);
            }
            Propagate::Next
        }
    };

    let mouse_handler = { move |_: MouseEvent| Propagate::Stop };

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

        (
            50,
            term_width - 50,
            26,
            term_height - 26,
        )
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
                            Text(text: format!("{}{} {}x{}", error_message.get(), if checkmate.get() { "\nCHECKMATE" } else if check.get() { "\nCHECK" } else { "" }, term_width, term_height))
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
