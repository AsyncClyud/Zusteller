use crate::api::headers::{clear_headers, remove_header};
use crate::{api::headers, ui::app::PostMan};

use eframe::egui::vec2;
use eframe::egui::{self};

pub fn show_header_builder(ui: &mut egui::Ui, app: &mut PostMan) {
    ui.add(
        egui::TextEdit::singleline(&mut app.header_key)
            .desired_rows(2)
            .desired_width(465.0)
            .hint_text("Header key")
            .char_limit(32),
    );
    ui.add(
        egui::TextEdit::singleline(&mut app.header_value)
            .desired_rows(2)
            .desired_width(465.0)
            .hint_text("Header value")
            .char_limit(128),
    );
    if ui
        .add(
            egui::Button::new("Add header")
                .fill(egui::Color32::BLACK)
                .min_size(vec2(30.0, 30.0)),
        )
        .clicked()
    {
        let _ = match headers::add_headers(app) {
            Ok(()) => app.header_message = String::from("Header added successfully"),
            Err(error) => app.header_message = error,
        };
    }
    show_headers(ui, app);
}

pub fn show_headers(ui: &mut egui::Ui, app: &mut PostMan) {
    egui::Frame::new().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Request headers");
            ui.small(format!("{} entries", app.headermap.len()));
            if ui.button("Clear all headers").clicked() {
                clear_headers(app)
            }
            ui.separator();
        });
        egui::ScrollArea::vertical()
            .id_salt("headers_scrollarea")
            .max_height(120.0)
            .max_width(220.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Grid::new("headers table")
                    .num_columns(2)
                    .min_col_width(120.0)
                    .striped(true)
                    .spacing([12.0, 7.0])
                    .show(ui, |ui| {
                        ui.strong("Key");
                        ui.strong("Value");
                        ui.end_row();

                        let headermap = &mut app.headermap;
                        let mut header_to_remove = Some("");

                        for (key, value) in headermap.iter() {
                            ui.label(key.as_str());
                            ui.label(value.to_str().unwrap());
                            if ui.button("Delete").clicked() {
                                header_to_remove = Some(key.as_str())
                            }
                            ui.end_row();
                        }
                        if let Some(key) = header_to_remove {
                            remove_header(headermap, key.to_string());
                        };
                    })
            })
    });
}
