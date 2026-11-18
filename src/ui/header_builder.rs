use crate::api::headers::{clear_headers, remove_header};
use crate::{api::headers, ui::app::PostMan};

use eframe::egui::vec2;
use eframe::egui::{self};
use rust_i18n::t;

pub fn show_header_builder(ui: &mut egui::Ui, app: &mut PostMan) {
    ui.add(
        egui::TextEdit::singleline(&mut app.header_key)
            .desired_rows(2)
            .desired_width(465.0)
            .hint_text(t!("headers.name").to_string())
            .char_limit(32),
    );
    ui.add(
        egui::TextEdit::singleline(&mut app.header_value)
            .desired_rows(2)
            .desired_width(465.0)
            .hint_text(t!("headers.value").to_string())
            .char_limit(128),
    );
    ui.add(egui::Label::new(app.header_message.clone()));
    if ui
        .add(
            egui::Button::new(t!("headers.add").to_string())
                .fill(egui::Color32::BLACK)
                .min_size(vec2(30.0, 30.0)),
        )
        .clicked()
    {
        let _ = match headers::add_headers(app) {
            Ok(()) => app.header_message = t!("headers.message").to_string(),
            Err(error) => app.header_message = error,
        };
    }
    show_headers(ui, app);
}

pub fn show_headers(ui: &mut egui::Ui, app: &mut PostMan) {
    egui::Frame::new().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading(t!("headers.title").to_string());
            ui.small(format!("{} entries", app.headermap.len()));
            if ui.button(t!("headers.remove.all").to_string()).clicked() {
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
                        ui.strong(t!("headers.name").to_string());
                        ui.strong(t!("headers.value").to_string());
                        ui.end_row();

                        let headermap = &mut app.headermap;
                        let mut header_to_remove = Some("");

                        for (key, value) in headermap.iter() {
                            ui.label(key.as_str());
                            ui.label(value.to_str().unwrap());
                            if ui.button(t!("headers.remove").to_string()).clicked() {
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
