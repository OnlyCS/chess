use std::collections::HashSet;

use crate::prelude::*;
use eframe::{
    egui,
    epaint::{Color32, Rounding, Stroke},
};

pub const LEN_SQ: f32 = 100.0;

#[derive(Clone, Copy)]
pub struct HighlightList {
    selected: Option<Square>,
    from: Option<Square>,
    to: Option<Square>,
}

impl HighlightList {
    fn new() -> Self {
        Self {
            selected: None,
            from: None,
            to: None,
        }
    }

    pub fn select(&mut self, sq: Square) {
        self.selected.replace(sq);
    }

    pub fn clear_select(&mut self) {
        self.selected = None;
    }

    pub fn from_to(&mut self, from: Square, to: Square) {
        self.from.replace(from);
        self.to.replace(to);
        self.selected = None;
    }

    pub fn highlighted(&self, sq: Square) -> bool {
        self.selected.is_some_and(|a| a == sq)
            || self.from.is_some_and(|a| a == sq)
            || self.to.is_some_and(|a| a == sq)
    }
}

impl Default for HighlightList {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SquareData<'a> {
    pub button: egui::Button<'a>,
    pub has_piece: bool,
    pub movable: bool,
    pub sq_idx: usize,
    pub on_click: fn(&mut ChessApp, bool, bool, Square),
}

impl<'a> Default for SquareData<'a> {
    fn default() -> Self {
        Self {
            button: egui::Button::new(""),
            has_piece: false,
            movable: false,
            sq_idx: 0,
            on_click: |_, _, _, _| {},
        }
    }
}

fn collect_data<'a>(app: &ChessApp) -> Vec<SquareData<'a>> {
    let position = &app.position;
    let selected = app.selected_piece;
    let highlight = &app.highlight;

    let mut movable_sqs = HashSet::new();
    let unordered = position.collect().into_iter().enumerate().collect_vec();
    let ordered = unordered.chunks(8).rev().flatten().copied().collect_vec();

    let mut data = vec![];

    if let Some(selected) = selected {
        movable_sqs = position.moves_of(selected).bit_pos_iter().collect();
    }

    for i in 0..64 {
        let (ipiece, piece) = ordered[i];
        let movable = movable_sqs.contains(&(ipiece as u8));

        let color = match (i % 2 == (i / 8) % 2, highlight.highlighted(ipiece as u8)) {
            (true, true) => Color32::from_rgb(244, 246, 128),
            (true, false) => Color32::from_rgb(233, 237, 204),
            (false, true) => Color32::from_rgb(187, 204, 68),
            (false, false) => Color32::from_rgb(119, 153, 84),
        };

        let mut button = if let Some(piece) = piece {
            let image = egui::Image::new(piece.image())
                .max_height(LEN_SQ)
                .max_width(LEN_SQ);

            egui::Button::image(image)
        } else {
            egui::Button::new("")
        };

        button = button
            .fill(color)
            .min_size(egui::vec2(LEN_SQ, LEN_SQ))
            .frame(false);

        fn round(a: bool) -> f32 {
            if a {
                10.0
            } else {
                0.0
            }
        }

        button = button.rounding(egui::Rounding {
            nw: round(i == 0),
            ne: round(i == 7),
            sw: round(i == 56),
            se: round(i == 63),
            ..Rounding::ZERO
        });

        data.push(SquareData {
            button,
            movable,
            has_piece: piece.is_some(),
            sq_idx: ipiece,
            on_click: |app, has_piece, movable, sq| {
                println!("clicked on {}", sq.pretty());

                if movable && let Some(piece) = app.selected_piece {
                    app.position.make_move(piece, sq);
                    app.highlight.from_to(piece, sq);
                } else if has_piece
                    && (app.selected_piece.is_some_and(|s| s != sq) || app.selected_piece.is_none())
                {
                    app.selected_piece = Some(sq);
                    app.highlight.select(sq);
                } else {
                    app.selected_piece = None;
                    app.highlight.clear_select();
                }
            },
        });
    }

    data
}

#[derive(Default)]
pub struct ChessApp {
    position: Position,
    selected_piece: Option<Square>,
    highlight: HighlightList,
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("chess_grid")
                .spacing(egui::vec2(0.0, 0.0))
                .min_col_width(50.0)
                .min_row_height(50.0)
                .show(ui, |ui| {
                    ui.input(|i| {
                        if i.key_pressed(egui::Key::ArrowLeft) {
                            self.position.undo_move();
                            self.highlight = HighlightList::default();
                            self.selected_piece = None;
                        }
                    });

                    let mut data = collect_data(self).into_iter();

                    for i in 0..64 {
                        ui.allocate_ui(egui::vec2(LEN_SQ, LEN_SQ), |ui| {
                            let square_data = data.next().unwrap();
                            let circle_pos = ui.min_rect().center() + egui::vec2(LEN_SQ / 2.0, 0.0);

                            if ui.add(square_data.button).clicked() {
                                (square_data.on_click)(
                                    self,
                                    square_data.has_piece,
                                    square_data.movable,
                                    square_data.sq_idx as u8,
                                );

                                ctx.request_repaint();
                            }

                            if square_data.movable {
                                if square_data.has_piece {
                                    ui.painter().circle_stroke(
                                        circle_pos,
                                        (LEN_SQ / 2.0) - (LEN_SQ / 20.0),
                                        Stroke::new(
                                            LEN_SQ / 10.0,
                                            Color32::from_rgba_premultiplied(128, 0, 0, 200),
                                        ),
                                    );
                                } else {
                                    ui.painter().circle_filled(
                                        circle_pos,
                                        LEN_SQ / 6.0,
                                        Color32::from_rgba_premultiplied(0, 0, 0, 25),
                                    );
                                }
                            }
                        });

                        if i % 8 == 7 {
                            ui.end_row();
                        }
                    }
                })
        });
    }
}
