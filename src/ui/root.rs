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
        .collect::<Vec<Vec<SelectData>>>()
        .iter()
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
                        } => selection.mutate(|s| s.hover.up()),
                        KeyEvent {
                            code: Char('a'), ..
                        } => selection.mutate(|s| s.hover.left()),
                        KeyEvent {
                            code: Char('s'), ..
                        } => selection.mutate(|s| s.hover.down()),
                        KeyEvent {
                            code: Char('d'), ..
                        } => selection.mutate(|s| s.hover.right()),
                        KeyEvent {
                            code: KeyCode::Enter,
                            ..
                        } => {
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
                        } => selection.mutate(|s| s.hover.up()),
                        KeyEvent {
                            code: Char('a'), ..
                        } => selection.mutate(|s| s.hover.left()),
                        KeyEvent {
                            code: Char('s'), ..
                        } => selection.mutate(|s| s.hover.down()),
                        KeyEvent {
                            code: Char('d'), ..
                        } => selection.mutate(|s| s.hover.right()),
                        KeyEvent { code: Esc, .. } => {
                            select_mode.set(SelectionMode::SelectPiece);

                            selection.mutate(|s| {
                                s.selected = None;
                                s.avaliable = vec![];
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
