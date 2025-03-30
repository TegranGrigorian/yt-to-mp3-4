use eframe::egui;
use std::sync::{Arc, Mutex};

pub fn download_screen(
    ui: &mut egui::Ui,
    input_url: &mut String, // Accept a mutable reference
    status_message: Arc<Mutex<String>>, // Accept the Arc<Mutex<String>> directly
    format: &String,
    video_type: &String,
    on_download: &mut dyn FnMut(),
    on_convert_again: &mut dyn FnMut(),
) {
    ui.heading(format!("Download {} as {}", video_type, format));

    // Input field for the YouTube URL
    ui.horizontal(|ui| {
        ui.label("YouTube URL:");
        ui.text_edit_singleline(input_url); // Use the mutable reference directly
    });

    // Download button
    if ui.button("Download").clicked() {
        on_download();
    }

    // Display the status message
    if let Ok(status) = status_message.lock() {
        ui.label(status.clone());
    }

    // Conditionally show the "Convert Again" button
    if let Ok(status) = status_message.lock() {
        if status.contains("Download complete!") {
            if ui.button("Convert Again").clicked() {
                on_convert_again();
            }
        }
    }
}