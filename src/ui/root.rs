use crossterm::event::MouseEvent;
use intuitive::{event::handler::Propagate, state::use_state, *};

use crate::{
    game::chess::Chess,
    parts::{position::Position, square::Square},
};

use super::{
    board::Board,
    data::{SelectData, UIFileData},
    selection::{Selection, SelectionMode},
};

#[component(Root)]
pub fn render() -> element::Any {
    let chess = use_state(Chess::default);
    let select_mode = use_state(|| SelectionMode::SelectPiece);
    let selection = use_state(|| Selection {
        hover: Position::default(),
        selected: None,
        avaliable: vec![],
    });

    let board_data = chess
        .get()
        .get_board()
        .get_files()
        .iter()
        .map(|f| f.clone().into_iter().collect::<Vec<Square>>())
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
        move |event| {
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
                        } => selection.mutate(|s| s.hover = s.hover.up_loop(1)),
                        KeyEvent {
                            code: Char('a'), ..
                        } => selection.mutate(|s| s.hover = s.hover.left_loop(1)),
                        KeyEvent {
                            code: Char('s'), ..
                        } => selection.mutate(|s| s.hover = s.hover.down_loop(1)),
                        KeyEvent {
                            code: Char('d'), ..
                        } => selection.mutate(|s| s.hover = s.hover.right_loop(1)),
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
                                _ => todo!("Error message"),
                            };

                            if selected_piece.get_color() != chess.get().get_turn() {
                                todo!("Error message");
                            }

                            selection.mutate(|s| {
                                s.selected = Some(s.hover.clone());
                                s.avaliable = selected_piece
                                    .get_moves(chess.get().get_board())
                                    .iter()
                                    .map(|m| m.to.clone())
                                    .collect();
                            });

                            select_mode.set(SelectionMode::SelectMove);
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
                        } => selection.mutate(|s| s.hover = s.hover.up_loop(1)),
                        KeyEvent {
                            code: Char('a'), ..
                        } => selection.mutate(|s| s.hover = s.hover.left_loop(1)),
                        KeyEvent {
                            code: Char('s'), ..
                        } => selection.mutate(|s| s.hover = s.hover.down_loop(1)),
                        KeyEvent {
                            code: Char('d'), ..
                        } => selection.mutate(|s| s.hover = s.hover.right_loop(1)),
                        KeyEvent { code: Esc, .. } => {
                            select_mode.set(SelectionMode::SelectPiece);

                            selection.mutate(|s| {
                                s.selected = None;
                                s.avaliable = vec![];
                            });
                        }
                        KeyEvent { code: Enter, .. } => {
                            chess.mutate(|game| {
                                let move_to = selection.get().hover;
                                let move_from = selection.get().selected.expect("Error message");
                                let available_movetos = selection.get().avaliable;
                                let mut moves = game.get_moves();

                                if !available_movetos.contains(&move_to) {
                                    todo!("Error message");
                                }

                                moves.retain(|x| x.from == move_from && x.to == move_to);
                                let mv = &moves[0];

                                let move_result = game.make_move(mv.clone());

                                if move_result.is_err() {
                                    todo!("Error message Err(e).tostring()?");
                                }

                                select_mode.set(SelectionMode::SelectPiece);

                                selection.mutate(|s| {
                                    s.selected = None;
                                    s.avaliable = vec![];
                                });
                            });
                        }
                        _ => (),
                    }
                }
            };
            Propagate::Next
        }
    };

    let mouse_handler = { move |_: MouseEvent| Propagate::Stop };

    render! {
        Board(on_key: key_hander, on_mouse: mouse_handler, board_data: board_data)
    }
}
