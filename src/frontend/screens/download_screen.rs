use eframe::egui;
use std::sync::{Arc, Mutex};

pub fn download_screen(
    ui: &mut egui::Ui,
    input_url: &mut String, // Accept a mutable reference to String
    status_message: Arc<Mutex<String>>,
    format: &String,
    video_type: &String,
    on_download: &mut dyn FnMut(),
    on_convert_again: &mut dyn FnMut(),
) {
    ui.heading(format!("Download {} as {}", video_type, format));

    // Example UI for input URL
    ui.horizontal(|ui| {
        ui.label("Input URL:");
        ui.text_edit_singleline(input_url); // Bind the text box directly to the mutable reference
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