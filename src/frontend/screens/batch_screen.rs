use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use dirs::{video_dir};
use egui::Context;
use crate::backend::batch_download::download_playlist_in_order;

pub struct BatchScreen {

}

impl BatchScreen {
    pub fn new() -> Self {
        BatchScreen {}
    }

    pub fn show_batch_screen(
        input_url: &mut String,
        status_message: Arc<Mutex<String>>,
        ctx: &Context,
        ui: &mut egui::Ui,
    ) {
        ui.heading("YouTube Playlist/Album Downloader");

        // Input field for the playlist/album URL
        ui.horizontal(|ui| {
            ui.label("Playlist/Album URL:");
            ui.text_edit_singleline(input_url);
        });

        // Button to start the batch download
        if ui.button("Download Playlist/Album").clicked() {
            println!("Batch download button clicked. Starting download...");

            // Check if the input URL is empty
            if input_url.is_empty() {
                if let Ok(mut status) = status_message.lock() {
                    *status = "Please enter a valid playlist or album URL.".to_string();
                }
            } else {
                if let Ok(mut status) = status_message.lock() {
                    *status = "Starting batch download...".to_string();
                }

                let url = input_url.clone();
                let video_dir = video_dir().unwrap_or_else(|| PathBuf::from("output"));
                let output_path = video_dir.join("yt-to-mp3-4");
                let status_message_clone = Arc::clone(&status_message);
                let ctx_clone = ctx.clone();

                // Spawn a new thread to run the batch download
                std::thread::spawn(move || {
                    let total_songs = {
                        let mut status = status_message_clone.lock().unwrap();
                        download_playlist_in_order(&url, "mp4", &output_path, &mut *status)
                    };
                    if let Ok(mut status) = status_message_clone.lock() {
                        *status = format!("Batch download completed: {}/{} songs.", total_songs, total_songs);
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