mod app;
mod requesting;

use eframe::egui::{self};

fn main() {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png"))
        .expect("Failed to load application icon");

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([800.0, 600.0])
            .with_icon(icon),
        centered: true,
        ..Default::default()
    };

    if let Err(error) = eframe::run_native(
        "Zusteller",
        options,
        Box::new(|_cc| Ok(Box::new(app::PostMan::default()))),
    ) {
        eprintln!("Error while opening app! {error}");
    }
}
