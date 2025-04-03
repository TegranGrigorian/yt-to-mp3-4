use eframe::egui;
use std::sync::{Arc, Mutex};
use std::path::PathBuf; // Import PathBuf
use rfd::FileDialog; // Import FileDialog for directory selection

pub fn download_screen(
    ui: &mut egui::Ui,
    input_url: &mut String, // Accept a mutable reference to String
    status_message: Arc<Mutex<String>>,
    format: &String,
    video_type: &String,
    output_dir: &mut PathBuf, // Add mutable reference to output directory
    on_download: &mut dyn FnMut(),
    on_convert_again: &mut dyn FnMut(),
) {
    ui.heading(format!("Download {} as {}", video_type, format));

    // Example UI for input URL
    ui.horizontal(|ui| {
        ui.label("Input URL:");
        ui.text_edit_singleline(input_url); // Bind the text box directly to the mutable reference
    });

    // Button to change the output directory
    if ui.button("Change Directory").clicked() {
        if let Some(dir) = FileDialog::new().pick_folder() {
            *output_dir = dir; // Update the output directory
            if let Ok(mut status) = status_message.lock() {
                *status = format!("Output directory changed to: {}", output_dir.display());
            }
        }
    }

    // Display the current output directory
    ui.label(format!("Output Directory: {}", output_dir.display()));

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