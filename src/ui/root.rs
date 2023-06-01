#![allow(clippy::unwrap_used)]

use crossterm::{
    event::{KeyEvent, MouseEvent},
    terminal::size,
};

use intuitive::{
    components::{self, *},
    element,
    event::handler::Propagate,
    state::use_state,
    *,
};

use crate::{
    core::{
        board::{Board, Event, GameEndedReason},
        color::Color,
        file::File,
        piece::PieceType,
        piece_move::MoveModifier,
        position::Position,
    },
    ui::{parts::BoardComponent, selection::Selection},
    utils::{string_builder::StringBuilder, traits::ToVec},
};

#[derive(Default)]
pub struct Root {
    starting_board: Board,
}

impl Root {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> components::Any {
        Self::default().into()
    }

    pub fn with_board(board: Board) -> components::Any {
        Self {
            starting_board: board,
        }
        .into()
    }
}

impl Component for Root {
    fn render(&self) -> element::Any {
        let game = use_state(|| self.starting_board.clone());
        let selection = use_state(|| Selection::SelectPiece(Position::default()));
        let game_ended = use_state(|| game.get().game_has_ended());
        let promotion_available = use_state(|| None::<Color>);
        let error_message = use_state(StringBuilder::default);
        let helper_text = use_state(StringBuilder::default);
        let check = use_state(|| None::<Color>);

        let key_handler = {
            let game = game.clone();
            let promotion = promotion_available;
            let game_ended = game_ended;
            let selection = selection.clone();
            let check = check;
            let helper_text = helper_text.clone();
            let error_message = error_message.clone();

            move |event: KeyEvent| {
                use intuitive::event::KeyCode::*;

                if let Some(reason) = game_ended.get() {
                    helper_text.mutate(|t| {
                        t.clear();
                        match reason {
                            GameEndedReason::Checkmate(col) => {
                                t.pushln(format!("{} won by checkmate", col.opposite()))
                            }
                            GameEndedReason::Stalemate(col) => {
                                t.pushln(format!("Draw by stalemate ({})", col))
                            }
                            GameEndedReason::FiftyMoveRule => t.addln("Draw by 50 move rule"),
                        }

                        t.addln("\nq to quit");
                    });

                    if let Char('q') = event.code {
                        event::quit();
                    }

                    return Propagate::Next;
                }

                if let Some(color) = promotion.get() {
                    promotion.set(None);

                    match event.code {
                        Char('b' | 'B') => game.mutate(|b| b.promote(color, PieceType::Bishop)),
                        Char('k' | 'K') => game.mutate(|b| b.promote(color, PieceType::Knight)),
                        Char('q' | 'Q') => game.mutate(|b| b.promote(color, PieceType::Queen)),
                        Char('r' | 'R') => game.mutate(|b| b.promote(color, PieceType::Rook)),
                        _ => error_message.mutate(|e| {
                            e.clear();
                            e.addln("Error: invalid promotion piece");

                            promotion.set(Some(color));
                            helper_text.mutate(|t| {
                                t.clear();

                                t.addln("\nPromote to one of these pieces:");
                                t.addln("\t(Q)ueen");
                                t.addln("\t(R)ook");
                                t.addln("\t(B)ishop");
                                t.addln("\t(K)night");
                            });
                        }),
                    }

                    return Propagate::Next;
                }

                match event.code {
                    Char('q') => event::quit(),
                    Char('w') | Up => selection.mutate(|s| s.up()),
                    Char('s') | Down => selection.mutate(|s| s.down()),
                    Char('a') | Left => selection.mutate(|s| s.left()),
                    Char('d') | Right => selection.mutate(|s| s.right()),
                    Enter => match selection.get() {
                        Selection::SelectPiece(hover_pos) => {
                            let mut board = game.get();

                            helper_text.mutate(|t| t.clear());

                            if let Some(piece) = board.square(hover_pos).and_then(|x| x.get_piece())
                            {
                                if piece.get_color() != board.turn() {
                                    error_message.mutate(|e| {
                                        e.clear();
                                        e.addln("Error: not your turn");
                                    });
                                    return Propagate::Next;
                                }
                            } else {
                                error_message.mutate(|e| {
                                    e.clear();
                                    e.addln("Error: No piece at selected location")
                                });
                                return Propagate::Next;
                            }

                            board.refresh();

                            selection.set(Selection::SelectMove(
                                hover_pos,
                                hover_pos,
                                board
                                    .get_moves()
                                    .iter()
                                    .filter(|m| hover_pos == m.from)
                                    .map(|x| x.to)
                                    .collect(),
                            ));

                            error_message.mutate(|e| e.clear());

                            if let Selection::SelectMove(_, _, available) = selection.get() {
                                if available.is_empty() {
                                    error_message.mutate(|e| {
                                        e.clear();
                                        e.addln("Warning: this piece has no moves");
                                        e.addln("Press Esc to start selecting a different piece");
                                    })
                                }
                            }
                        }
                        Selection::SelectMove(hover_pos, piece_pos, moves) => {
                            game.mutate(|board| {
                                let selected_position =
                                    match moves.iter().find(|m| **m == hover_pos) {
                                        Some(pos) => pos,
                                        None => {
                                            error_message.mutate(|e| {
                                                e.clear();
                                                e.addln("Error: invalid move");
                                            });
                                            return;
                                        }
                                    };

                                let moves = board.get_moves();

                                let selected_move = moves
                                    .iter()
                                    .find(|m| m.to == *selected_position && m.from == piece_pos)
                                    .unwrap();

                                if let Err(err) = board.make_move(selected_move) {
                                    error_message.mutate(|e| {
                                        e.clear();
                                        e.pushln(format!("Move failed: {}", err))
                                    });
                                    return;
                                }

                                board.event_emitter.emit(Event::Move, selected_move.clone());

                                selection.set(Selection::SelectPiece(hover_pos));
                                error_message.mutate(|e| e.clear());

                                // if there is a PromotionUnknown(Color) in selected_move.movemodifiers, get the color
                                if let Some(MoveModifier::PromotionUnknown(col)) = selected_move
                                    .modifiers
                                    .iter()
                                    .find(|x| matches!(**x, MoveModifier::PromotionUnknown(_)))
                                {
                                    promotion.set(Some(*col));

                                    helper_text.mutate(|t| {
                                        t.clear();

                                        t.addln("Promote to one of these pieces:");
                                        t.addln("\t(Q)ueen");
                                        t.addln("\t(R)ook");
                                        t.addln("\t(B)ishop");
                                        t.addln("\t(K)night");
                                    });
                                }

                                check.set(board.is_check());
                            });
                        }
                    },
                    Esc => {
                        error_message.mutate(|e| e.clear());
                        selection.set(Selection::SelectPiece(match selection.get() {
                            Selection::SelectMove(x, _, _) => x,
                            Selection::SelectPiece(x) => x,
                        }))
                    }
                    _ => {}
                };

                Propagate::Next
            }
        };

        let mouse_handler = move |_: MouseEvent| Propagate::Stop;

        // prerender section
        let mut direction = helper_text.get().to_string();
        let (term_w, term_h) = size().unwrap();

        let min_term_w = 100;
        let min_term_h = 28;
        let size_ok = term_w >= min_term_w && term_h >= min_term_h;

        let flex = if size_ok {
            let mut wasdtext: String = String::default();
            wasdtext += "WASD/Arrow Keys to move";
            wasdtext += "\nEnter to select move/piece";
            wasdtext += "\nq to quit";
            direction = format!("{}\n{}\n{}", wasdtext, direction, error_message.get());

            (50, term_w - 50, 26, term_h - 26)
        } else {
            direction = String::new();
            direction += "Increase terminal size";

            (0, 1, 1, 0)
        };

        let board = game.get();
        let borrowed_files: Vec<&File> = board.to_vec();
        let mut files: Vec<File> = borrowed_files.iter().copied().cloned().collect::<Vec<_>>();

        for elem in files.iter_mut() {
            elem.squares.reverse()
        }

        render! {
            VStack(on_key: key_handler, on_mouse: mouse_handler, flex: [flex.2, flex.3]) {
                HStack(flex: [flex.0, flex.1]) {
                    Section(title: "Board") {
                        BoardComponent(board: files, selection: selection.get())
                    }

                    Section(title: "Instructions") {
                        Text(text: direction)
                    }
                }

                Empty()
            }
        }
    }
}
