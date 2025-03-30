use crate::frontend::components::button;
use eframe::egui;

pub fn main_window(ui: &mut egui::Ui, input_url: &mut String, status_message: &mut String) {
    ui.heading("YouTube to MP3 Converter");

    ui.horizontal(|ui| {
        ui.label("YouTube URL:");
        ui.text_edit_singleline(input_url);
    });

    if button::custom_button(ui, "Download MP3").clicked() {
        if input_url.is_empty() {
            *status_message = "Please enter a valid URL.".to_string();
        } else {
            *status_message = "Downloading...".to_string();
        }
    }

    ui.label(status_message);
}