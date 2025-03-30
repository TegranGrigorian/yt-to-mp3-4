use std::sync::{Arc, Mutex};
use std::thread;
use crate::backend;

pub fn handle_download(
    input_url: String,
    format: String,
    video_type: String,
    status_message: Arc<Mutex<String>>,
) {
    // Spawn a new thread to handle the download
    thread::spawn(move || {
        // Update the status message to "Downloading {Video Name}"
        if let Ok(mut status) = status_message.lock() {
            *status = format!("Downloading {} as {}", video_type, format);
        }

        // Simulate the download process (replace this with actual backend logic)
        let result = backend::Mp3Convert::ConvertMp3::ConvertMp3::new(input_url.clone(), format.clone()).convert();

        // Update the status message based on the result
        if let Ok(mut status) = status_message.lock() {
            *status = match result {
                Ok(_) => format!("Download complete!"),
                Err(err) => format!("Download failed: {}", err),
            };
        }
    });
}

pub fn download_screen(
    ui: &mut egui::Ui,
    input_url: &mut String,
    status_message: &mut String,
    format: &String,
    video_type: &String,
    on_download: &mut dyn FnMut(),
    on_convert_again: &mut dyn FnMut(), // Callback for "Convert Again"
) {
    ui.heading(format!("Download {} as {}", video_type, format));

    // Input field for the YouTube URL
    ui.horizontal(|ui| {
        ui.label("YouTube URL:");
        ui.text_edit_singleline(input_url);
    });

    // Download button
    if ui.button("Download").clicked() {
        on_download();
    }

    // Display the status message
    ui.label(status_message.clone());

    // Conditionally show the "Convert Again" button
    if status_message.contains("Download complete!") {
        if ui.button("Convert Again").clicked() {
            on_convert_again();
        }
    }
}