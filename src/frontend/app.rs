use eframe::egui;
use crate::backend::Mp3Convert::ConvertMp3;

pub struct App {
    input_url: String,
    status_message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input_url: String::new(),
            status_message: String::from("Enter a YouTube URL to start."),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("YouTube to MP3 Converter");

            ui.horizontal(|ui| {
                ui.label("YouTube URL:");
                ui.text_edit_singleline(&mut self.input_url);
            });

            if ui.button("Download MP3").clicked() {
                if self.input_url.is_empty() {
                    self.status_message = "Please enter a valid URL.".to_string();
                } else {
                    self.status_message = "Downloading...".to_string();
                    let url = self.input_url.clone();
                    tokio::spawn(async move {
                        let convert_mp3 = ConvertMp3::ConvertMp3::new(url, "output.mp3".to_string());
                        convert_mp3.convert().await;
                    });
                }
            }

            ui.label(&self.status_message);
        });
    }
}