use std::sync::{Arc, Mutex};
use std::thread;
use std::path::PathBuf; // Import PathBuf
use rfd::FileDialog; // Import FileDialog for directory selection
use crate::backend;

pub fn handle_download(
    input_url: String,
    format: String,
    video_type: String,
    output_dir: PathBuf, // Add output directory parameter
    status_message: Arc<Mutex<String>>,
    ctx: egui::Context, // Pass the egui context
) {
    // Spawn a new thread to handle the download
    std::thread::spawn(move || {
        // Update the status message to "Downloading {Video Name}"
        if let Ok(mut status) = status_message.lock() {
            *status = format!("Downloading {} as {}", video_type, format);
        }
        ctx.request_repaint(); // Force GUI repaint

        // Determine the conversion logic based on the format
        let result = match format.as_str() {
            "MP3" => backend::Mp3Convert::ConvertMp3::ConvertMp3::new(
                input_url.clone(),
                output_dir, // Use the selected output directory, no join this is handeled in the the backend``
            )
            .convert(),
            "MP4" => backend::Mp4Convert::ConvertMp4::ConvertMp4::new(
                input_url.clone(),
                output_dir,
            )
            .convert(),
            _ => Err("Unsupported format".to_string()),
        };

        // Update the status message based on the result
        if let Ok(mut status) = status_message.lock() {
            *status = match result {
                Ok(_) => format!("Download complete!"),
                Err(err) => format!("Download failed: {}", err),
            };
        }
        ctx.request_repaint(); // Force GUI update
    });
}

pub fn download_screen(
    ui: &mut egui::Ui,
    input_url: &mut String,
    status_message: &mut String,
    format: &String,
    video_type: &String,
    output_dir: &mut PathBuf, // Add mutable reference to output directory
    on_download: &mut dyn FnMut(),
    on_convert_again: &mut dyn FnMut(), // Callback for "Convert Again"
) {
    ui.heading(format!("Download {} as {}", video_type, format));

    // Input field for the YouTube URL
    ui.horizontal(|ui| {
        ui.label("YouTube URL:");
        ui.text_edit_singleline(input_url);
    });

    // Button to change the output directory
    if ui.button("Change Directory").clicked() {
        if let Some(dir) = FileDialog::new().pick_folder() {
            *output_dir = dir; // Update the output directory
            *status_message = format!("Output directory changed to: {}", output_dir.display());
        }
    }

    // Display the current output directory
    ui.label(format!("Output Directory: {}", output_dir.display()));

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