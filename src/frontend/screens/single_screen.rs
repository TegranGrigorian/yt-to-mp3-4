use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use dirs::audio_dir;
use egui::Context;
use crate::backend::Mp3Convert::ConvertMp3;

pub struct SingleScreen {

}

impl SingleScreen {
    pub fn new() -> Self {
        SingleScreen {}
    }

    pub fn show_single_screen(
        input_url: &mut String,
        status_message: Arc<Mutex<String>>,
        ctx: &Context,
        ui: &mut egui::Ui,
    ) {
        ui.heading("YouTube to MP3 Converter");

        // Input field for the YouTube URL
        ui.horizontal(|ui| {
            ui.label("YouTube URL:");
            ui.text_edit_singleline(input_url);
        });

        // Button to start the download
        if ui.button("Download MP3").clicked() {
            println!("Button clicked. Starting download...");

            // Check if the input URL is empty
            if input_url.is_empty() {
                if let Ok(mut status) = status_message.lock() {
                    *status = "Please enter a valid URL.".to_string();
                }
            } else {
                if let Ok(mut status) = status_message.lock() {
                    *status = "Downloading...".to_string();
                }

                let url = input_url.clone();
                let music_dir = audio_dir().unwrap_or_else(|| PathBuf::from("output"));
                let output_path = music_dir.join("output.mp3");
                let status_message_clone = Arc::clone(&status_message);
                let ctx_clone = ctx.clone();

                // Spawn a new thread to run the `convert` method
                std::thread::spawn(move || {
                    let convert_mp3 = ConvertMp3::ConvertMp3::new(url, PathBuf::from(&output_path));
                    let mut new_status_message = String::new();
                    match convert_mp3.convert() {
                        Ok(_) => {
                            new_status_message = format!(
                                "Download complete! File saved to: {}",
                                output_path.display()
                            );
                        }
                        Err(err) => {
                            new_status_message = format!("Download failed: {}", err);
                        }
                    }
                    if let Ok(mut status) = status_message_clone.lock() {
                        *status = new_status_message;
                    }

                    ctx_clone.request_repaint();
                });
            }
        }

        // Display the status message
        if let Ok(status) = status_message.lock() {
            ui.label(status.clone());
        }
    }
    
}