use eframe::egui;
use rust_i18n::t;

use crate::ui::app::PostMan;

pub fn show_body_builder(ui: &mut egui::Ui, app: &mut PostMan) {
    ui.add(egui::Label::new(
        egui::RichText::new(t!("request.body").to_string()).color(egui::Color32::WHITE),
    ));

    ui.add(
        egui::TextEdit::multiline(&mut app.body)
            .desired_width(400.0)
            .desired_rows(4)
            .hint_text(format!("{:?}: {:?}", "Article_ID", "320"))
            .char_limit(128),
    );
}
