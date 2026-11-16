use eframe::egui;

pub fn show_settings_window(ui: &egui::Ui, show_settings: &mut bool) {
    egui::Window::new("")
        .open(show_settings)
        .default_size([400.0, 500.0])
        .fixed_size([400.0, 500.0])
        .resizable(true)
        .collapsible(false)
        .show(ui.ctx(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading(
                    egui::RichText::new("Settings")
                        .color(egui::Color32::WHITE)
                        .strong(),
                );
                ui.label("No settings yet. //ToDo: Make a custom themes;")
            })
        });
}
