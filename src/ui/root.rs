use intuitive::{components::*, event::handler::Propagate, state::use_state, *};

use crate::{game::chess::Chess, parts::position::Position};

use super::selection::{Selection, SelectionMode};

#[component(Root)]
pub fn render() {
    // define a game
    let game = Chess::default();

    // create states for storing selections
    let select_mode = use_state(|| SelectionMode::SelectPiece);
    let selection = use_state(|| Selection {
        hover: Position::default(),
        selected: None,
        avaliable: vec![],
    });

    // define key handler
    let key_hander = {
        use super::selection::SelectionMode;

        move |event| {
            match select_mode.get() {
                SelectionMode::SelectPiece => {
                    use intuitive::event::{
                        self,
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
                            selection.mutate(|s| {
                                s.selected = Some(s.hover.copy());
                                s.avaliable = game
                                    .get_board()
                                    .square(&s.hover)
                                    .expect("No piece at selected position")
                                    .get_piece()
                                    .expect("No piece at selected position")
                                    .get_moves(game.get_board())
                                    .iter()
                                    .map(|p| p.clone().to)
                                    .collect();
                            });
                        }

                        _ => (),
                    }
                }

                SelectionMode::SelectMove => {
                    use intuitive::event::{self, KeyCode::*, KeyEvent};

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

    render! {
        Text(text: "Hello, world!")
    }
}
