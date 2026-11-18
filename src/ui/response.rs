use eframe::egui;
use rust_i18n::t;

pub fn show_response(ui: &mut egui::Ui, response: String) {
    egui::Frame::new()
        .corner_radius(4.0)
        .inner_margin(5.0)
        .shadow(egui::Shadow::default())
        .show(ui, |ui| {
            ui.add(egui::Label::new(
                egui::RichText::new(t!("response.title").to_string()).color(egui::Color32::WHITE),
            ));
            egui::Frame::new()
                .fill(egui::Color32::BLACK)
                .corner_radius(8.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("response_scrollarea")
                        .max_height(350.0)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(response).color(egui::Color32::WHITE),
                                )
                                .wrap(),
                            );
                        });
                });
        });
}
