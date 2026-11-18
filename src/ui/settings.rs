use crate::ui::app::PostMan;

use eframe::egui;
use rust_i18n::t;

pub fn show_settings_window(ui: &egui::Ui, app: &mut PostMan) {
    egui::Window::new("")
        .open(&mut app.show_settings)
        .default_size([400.0, 500.0])
        .fixed_size([400.0, 500.0])
        .resizable(true)
        .collapsible(false)
        .show(ui.ctx(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading(
                    egui::RichText::new(t!("settings.title").to_string())
                        .color(egui::Color32::WHITE)
                        .strong(),
                );
                ui.add(egui::Label::new(t!("settings.language").to_string()));

                let mut language_changed = false;

                egui::ComboBox::from_id_salt("language_selector")
                    .selected_text(&app.settings.language)
                    .width(100.0)
                    .show_ui(ui, |ui| {
                        language_changed |= ui
                            .selectable_value(
                                &mut app.settings.language,
                                String::from("en"),
                                "English",
                            )
                            .changed();
                        language_changed |= ui
                            .selectable_value(
                                &mut app.settings.language,
                                String::from("de"),
                                "Deutsch",
                            )
                            .changed();
                        language_changed |= ui
                            .selectable_value(
                                &mut app.settings.language,
                                String::from("ru"),
                                "Русский",
                            )
                            .changed();
                    });
                if language_changed {
                    change_language(app.settings.language.clone());
                }
            })
        });
}

fn change_language(language: String) {
    rust_i18n::set_locale(language.as_str());
}
