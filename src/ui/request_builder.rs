use crate::api::requesting;
use crate::ui::app;
use crate::ui::body_builder::show_body_builder;
use crate::ui::header_builder::show_header_builder;
use eframe::egui::vec2;
use eframe::egui::{self};

pub fn show_request_builder(ui: &mut egui::Ui, app: &mut app::PostMan) {
    egui::Frame::new()
        .inner_margin(5.0)
        .corner_radius(10.0)
        .shadow(egui::Shadow::default())
        .show(ui, |ui| {
            egui::Ui::horizontal(ui, |ui| {
                egui::ComboBox::from_id_salt("")
                    .selected_text(format!("{}", app.method))
                    .width(50.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut app.method, String::from("GET"), "GET");
                        ui.selectable_value(&mut app.method, String::from("POST"), "POST");
                        ui.selectable_value(&mut app.method, String::from("PUT"), "PUT");
                        ui.selectable_value(&mut app.method, String::from("PATCH"), "PATCH");
                        ui.selectable_value(&mut app.method, String::from("DELETE"), "DELETE");
                    });
                ui.add(
                    egui::TextEdit::singleline(&mut app.adress)
                        .hint_text("https://example.com/get_something")
                        .desired_rows(2)
                        .desired_width(400.0),
                )
            });
            ui.add(egui::Label::new(
                egui::RichText::new("Headers").color(egui::Color32::WHITE),
            ));
            show_header_builder(ui, app);
            show_body_builder(ui, app);
            if ui
                .add(
                    egui::Button::new("Send")
                        .fill(egui::Color32::BLACK)
                        .min_size(vec2(30.0, 30.0)),
                )
                .clicked()
            {
                requesting::requesting(app);
            }
        });
    if let Some(receiver) = &app.receiver {
        if let Ok(result) = receiver.try_recv() {
            match result {
                Ok(body) => app.response = body,
                Err(error) => app.response = error,
            }
            app.receiver = None
        }
    }
}
