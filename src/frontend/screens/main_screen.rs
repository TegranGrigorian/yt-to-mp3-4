use crate::frontend::components::button;
use eframe::egui;
use std::path::PathBuf;

pub fn main_window(
    ui: &mut egui::Ui,
    input_url: &mut String,
    status_message: &mut String,
    format: &mut String,       // To store the selected format (MP3 or MP4)
    output_dir: &mut PathBuf,  // To store the selected output directory
) {
    ui.heading("YouTube to MP3/MP4 Converter");

    // Input field for the YouTube URL
    ui.horizontal(|ui| {
        ui.label("YouTube URL:");
        ui.text_edit_singleline(input_url);
    });

    // Dropdown for selecting the format
    ui.horizontal(|ui| {
        ui.label("Select Format:");
        egui::ComboBox::from_label("")
            .selected_text(format.clone())
            .show_ui(ui, |ui| {
                ui.selectable_value(format, "MP3".to_string(), "MP3");
                ui.selectable_value(format, "MP4".to_string(), "MP4");
            });
    });

    // Button to select the output directory
    ui.horizontal(|ui| {
        ui.label("Output Directory:");
        if button::custom_button(ui, "Choose Directory").clicked() {
            // Placeholder: Replace this with actual directory selection logic
            *output_dir = PathBuf::from("C:/Users/Default/Downloads");
            *status_message = format!("Output directory set to: {}", output_dir.display());
        }
        ui.label(output_dir.display().to_string());
    });

    // Button to start the download
    if button::custom_button(ui, "Download").clicked() {
        if input_url.is_empty() {
            *status_message = "Please enter a valid URL.".to_string();
        } else if output_dir.as_os_str().is_empty() {
            *status_message = "Please select an output directory.".to_string();
        } else {
            *status_message = format!("Downloading as {}...", format);
        }
    }

    // Display the status message
    ui.label(&status_message.clone());
}