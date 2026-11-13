use crate::api::{headers, requesting};
use eframe::egui::{self};
use reqwest::header::HeaderMap;
use std::sync::mpsc::Receiver;

pub struct PostMan {
    show_settings: bool,
    pub adress: String,
    pub method: String,
    pub headermap: HeaderMap,
    pub header_key: String,
    pub header_value: String,
    pub header_message: String,
    pub body: String,
    pub client: reqwest::Client,
    pub response: String,
    pub receiver: Option<Receiver<Result<String, String>>>,
}

impl Default for PostMan {
    fn default() -> Self {
        Self {
            show_settings: false,
            adress: String::from("https://httpbin.org/user-agent"),
            method: String::from("GET"),
            headermap: HeaderMap::new(),
            header_key: String::from("User-Agent"),
            header_value: String::from("Zusteller/0.0.1"),
            header_message: String::from(""),
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
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Settings", |ui| {
                    if ui
                        .add(egui::Button::new("Settings").fill(egui::Color32::BLACK))
                        .clicked()
                    {
                        self.show_settings = true;
                    }
                })
            });
            ui.vertical_centered_justified(|ui| {
                ui.heading(
                    egui::RichText::new("Zusteller")
                        .color(egui::Color32::WHITE)
                        .strong(),
                );
                ui.add(egui::Separator::default())
            });
            ui.vertical_centered(|ui| {
                ui.add_space(35.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("Adress").color(egui::Color32::WHITE),
                ));
                ui.add(
                    egui::TextEdit::singleline(&mut self.adress)
                        .desired_width(305.0)
                        .hint_text("https://example.com")
                        .char_limit(64),
                );
                ui.add(egui::Label::new(
                    egui::RichText::new("Method").color(egui::Color32::WHITE),
                ));
                egui::ComboBox::from_id_salt("")
                    .selected_text(format!("{}", self.method))
                    .width(300.0)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.method, String::from("GET"), "GET");
                        ui.selectable_value(&mut self.method, String::from("POST"), "POST");
                        ui.selectable_value(&mut self.method, String::from("PUT"), "PUT");
                        ui.selectable_value(&mut self.method, String::from("PATCH"), "PATCH");
                        ui.selectable_value(&mut self.method, String::from("DELETE"), "DELETE");
                    });
                ui.add(egui::Label::new(
                    egui::RichText::new("Headers").color(egui::Color32::WHITE),
                ));
                ui.add(
                    egui::TextEdit::singleline(&mut self.header_key)
                        .hint_text("Header key")
                        .char_limit(32),
                );
                ui.add(
                    egui::TextEdit::singleline(&mut self.header_value)
                        .hint_text("Header value")
                        .char_limit(128),
                );
                if ui
                    .add(egui::Button::new("Add header").fill(egui::Color32::BLACK))
                    .clicked()
                {
                    let _ = match headers::add_headers(self) {
                        Ok(()) => self.header_message = String::from("Headers added successfully"),
                        Err(error) => self.header_message = error,
                    };
                }
                ui.add(egui::Label::new(egui::RichText::new(&self.header_message)));
                ui.add(egui::Label::new(
                    egui::RichText::new("Body").color(egui::Color32::WHITE),
                ));

                ui.add(
                    egui::TextEdit::multiline(&mut self.body)
                        .desired_width(300.0)
                        .desired_rows(4)
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
                            .max_height(100.0)
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
                    });
                // Show menu window if show_about argument is true
                if self.show_settings {
                    egui::Window::new("")
                        .open(&mut self.show_settings)
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
                                ui.label("No settings yet. ToDo: Make a custom themes;")
                            })
                        });
                }
            });
        });
    }
}
