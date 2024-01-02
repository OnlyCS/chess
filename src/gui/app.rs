use std::collections::HashSet;

use crate::prelude::*;
use eframe::{
    egui,
    epaint::{Color32, Rounding, Stroke},
};

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

fn collect_data<'a>(position: Position, selected: Option<Square>) -> Vec<SquareData<'a>> {
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

        let color = if i % 2 == (i / 8) % 2 {
            Color32::from_rgb(255, 206, 158)
        } else {
            Color32::from_rgb(209, 139, 71)
        };

        let mut button = if let Some(piece) = piece {
            let image = egui::Image::new(piece.image())
                .max_height(50.0)
                .max_width(50.0);

            egui::Button::image(image)
        } else {
            egui::Button::new("")
        };

        button = button
            .fill(color)
            .stroke(egui::Stroke::new(2.0, Color32::BLACK))
            .min_size(egui::vec2(50.0, 50.0))
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

                if movable {
                    app.position.make_move(app.selected_piece.unwrap(), sq);
                } else if has_piece {
                    app.selected_piece = Some(sq);
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
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("chess_grid")
                .spacing(egui::vec2(0.0, 0.0))
                .min_col_width(50.0)
                .min_row_height(50.0)
                .show(ui, |ui| {
                    let mut data = collect_data(self.position, self.selected_piece).into_iter();

                    for i in 0..64 {
                        ui.allocate_ui(egui::vec2(50.0, 50.0), |ui| {
                            let square_data = data.next().unwrap();
                            let mut circle_pos = ui.min_rect().center();

                            circle_pos.x += 25.0;

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
                                ui.painter().circle(
                                    circle_pos,
                                    10.0,
                                    Color32::from_rgba_unmultiplied(105, 105, 105, 150),
                                    Stroke::NONE,
                                )
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
