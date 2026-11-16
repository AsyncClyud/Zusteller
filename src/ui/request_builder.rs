use crate::api::headers;
use crate::api::requesting;
use crate::ui::app;
use eframe::egui::{self};

pub fn show_request_builder(ui: &mut egui::Ui, app: &mut app::PostMan) {
    egui::Frame::new()
        .fill(egui::Color32::DARK_GRAY)
        .corner_radius(6.5)
        .inner_margin(9.0)
        .shadow(egui::Shadow::default())
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(35.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("Adress").color(egui::Color32::WHITE),
                ));
                ui.add(
                    egui::TextEdit::singleline(&mut app.adress)
                        .desired_width(305.0)
                        .hint_text("https://example.com")
                        .char_limit(64),
                );
                ui.add(egui::Label::new(
                    egui::RichText::new("Method").color(egui::Color32::WHITE),
                ));
                egui::ComboBox::from_id_salt("")
                    .selected_text(format!("{}", app.method))
                    .width(300.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.method, String::from("GET"), "GET");
                        ui.selectable_value(&mut app.method, String::from("POST"), "POST");
                        ui.selectable_value(&mut app.method, String::from("PUT"), "PUT");
                        ui.selectable_value(&mut app.method, String::from("PATCH"), "PATCH");
                        ui.selectable_value(&mut app.method, String::from("DELETE"), "DELETE");
                    });
                ui.add(egui::Label::new(
                    egui::RichText::new("Headers").color(egui::Color32::WHITE),
                ));
                ui.add(egui::Label::new(
                    egui::RichText::new(headers::show_current_headers(app.headermap.clone()))
                        .color(egui::Color32::WHITE),
                ));
                ui.add(
                    egui::TextEdit::singleline(&mut app.header_key)
                        .hint_text("Header key")
                        .char_limit(32),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut app.header_value)
                        .hint_text("Header value")
                        .char_limit(128),
                );
                if ui
                    .add(egui::Button::new("Add header").fill(egui::Color32::BLACK))
                    .clicked()
                {
                    let _ = match headers::add_headers(app) {
                        Ok(()) => app.header_message = String::from("Header added successfully"),
                        Err(error) => app.header_message = error,
                    };
                }
                ui.add(egui::Label::new(
                    egui::RichText::new(&app.header_message).color(egui::Color32::WHITE),
                ));
                ui.add(egui::Label::new(
                    egui::RichText::new("Body").color(egui::Color32::WHITE),
                ));

                ui.add(
                    egui::TextEdit::multiline(&mut app.body)
                        .desired_width(300.0)
                        .desired_rows(4)
                        .hint_text(format!("{:?}: {:?}", "Article_ID", "320"))
                        .char_limit(128),
                );
                ui.add_space(10.0);
                if ui
                    .add(egui::Button::new("Send").fill(egui::Color32::BLACK))
                    .clicked()
                {
                    requesting::requesting(app);
                }
                if let Some(receiver) = &app.receiver {
                    if let Ok(result) = receiver.try_recv() {
                        match result {
                            Ok(body) => app.response = body,
                            Err(error) => app.response = error,
                        }
                        app.receiver = None
                    }
                }
            });
        });
}
