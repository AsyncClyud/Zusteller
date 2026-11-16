use crate::ui::{request_builder, response, settings};
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
    pub runtime: tokio::runtime::Runtime,
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
            header_value: String::from("Zusteller/0.1.0"),
            header_message: String::from(""),
            body: String::from(""),
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .worker_threads(2)
                .build()
                .expect("Error while creating a tokio::runtime"),
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
                });
            });

            request_builder::show_request_builder(ui, self);

            ui.add_space(15.0);
            response::show_response(ui, self.response.clone());
            // Show settings window if show_settings argument is true
            if self.show_settings {
                settings::show_settings_window(ui, &mut self.show_settings);
            }
        });
    }
}
