use crate::ui::{request_builder, response, settings};
use eframe::egui::{self};
use reqwest::header::HeaderMap;
use rust_i18n::t;
use std::sync::mpsc::Receiver;

// ToDo: make a storage
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppSettings {
    pub language: String,
}

pub struct PostMan {
    pub adress: String,
    pub method: String,
    pub headermap: HeaderMap,
    pub header_key: String,
    pub header_value: String,
    pub header_message: String,
    pub body: String,
    pub settings: AppSettings,
    pub show_settings: bool,
    pub runtime: tokio::runtime::Runtime,
    pub client: reqwest::Client,
    pub response: String,
    pub receiver: Option<Receiver<Result<String, String>>>,
}

impl Default for PostMan {
    fn default() -> Self {
        Self {
            adress: String::from(""),
            method: String::from("GET"),
            headermap: HeaderMap::new(),
            header_key: String::from("User-Agent"),
            header_value: String::from("Zusteller/0.5.0"),
            header_message: String::from(""),
            body: String::from(""),
            settings: AppSettings {
                language: String::from("en"),
            },
            show_settings: false,
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

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: String::from("en"),
        }
    }
}

impl eframe::App for PostMan {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button(t!("settings.title").to_string(), |ui| {
                    if ui
                        .add(
                            egui::Button::new(t!("settings.title").to_string())
                                .fill(egui::Color32::BLACK),
                        )
                        .clicked()
                    {
                        self.show_settings = true;
                    }
                });
            });

            request_builder::show_request_builder(ui, self);
            response::show_response(ui, self.response.clone());
            // Show settings window if show_settings argument is true
            if self.show_settings {
                settings::show_settings_window(ui, self);
            }
        });
    }
}
