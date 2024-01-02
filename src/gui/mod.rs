use eframe::egui;

mod app;
pub mod gui_piece;

pub fn run() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([415.0, 415.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Chess",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<app::ChessApp>::default()
        }),
    )
}
