use eframe::egui;

mod app;
pub mod gui_piece;

pub fn run() -> Result<(), eframe::Error> {
    let vp_size = (8.0 * app::LEN_SQ) + 15.0;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([vp_size, vp_size]),
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
