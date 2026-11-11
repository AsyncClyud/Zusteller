use crate::requesting;
use eframe::egui::{self};
use std::sync::mpsc::Receiver;

pub struct PostMan {
    pub adress: String,
    pub method: String,
    pub body: String,
    pub client: reqwest::Client,
    pub response: String,
    pub receiver: Option<Receiver<Result<String, String>>>,
}

impl Default for PostMan {
    fn default() -> Self {
        Self {
            adress: String::from(""),
            method: String::from("GET"),
            body: String::from(""),
            client: reqwest::Client::new(),
            response: String::from(""),
            receiver: None,
        }
    }
}

impl eframe::App for PostMan {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading(egui::RichText::new("Zusteller").color(egui::Color32::BROWN));
                ui.add(egui::Separator::default())
            });
            ui.vertical_centered(|ui| {
                ui.add_space(35.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("Adress").color(egui::Color32::WHITE),
                ));
                ui.add(
                    egui::TextEdit::singleline(&mut self.adress)
                        .hint_text("https://example.com")
                        .char_limit(64),
                );
                ui.add(egui::Label::new(
                    egui::RichText::new("Method").color(egui::Color32::WHITE),
                ));
                ui.add(
                    egui::TextEdit::singleline(&mut self.method)
                        .hint_text("GET, POST, PUT, DELETE.")
                        .char_limit(8),
                );
                ui.add(egui::Label::new(
                    egui::RichText::new("Body").color(egui::Color32::WHITE),
                ));

                ui.add(
                    egui::TextEdit::multiline(&mut self.body)
                        .hint_text(format!("{}: {}", "Article_ID", "320"))
                        .char_limit(128),
                );
                ui.add_space(10.0);
                if ui
                    .add(egui::Button::new("Send").fill(egui::Color32::BLACK))
                    .clicked()
                {
                    requesting::requesting(self);
                }
                if let Some(receiver) = &self.receiver {
                    if let Ok(result) = receiver.try_recv() {
                        match result {
                            Ok(body) => self.response = body,
                            Err(error) => self.response = error,
                        }
                        self.receiver = None
                    }
                }
                ui.add_space(20.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("Response").color(egui::Color32::WHITE),
                ));
                egui::Frame::new()
                    .fill(egui::Color32::BLACK)
                    .corner_radius(8.0)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&self.response)
                                            .color(egui::Color32::WHITE),
                                    )
                                    .wrap(),
                                );
                            });
                        if ui
                            .add(egui::Button::new("Copy Response").fill(egui::Color32::DARK_GRAY))
                            .clicked()
                        {
                            ui.ctx().copy_text(self.response.clone());
                        }
                    })
            });
        });
    }
}
