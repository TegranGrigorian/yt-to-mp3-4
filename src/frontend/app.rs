use std::path::PathBuf;
use dirs::audio_dir;
use crate::backend::{self, Mp3Convert::ConvertMp3::ConvertMp3};

pub struct App {
    pub input_url: String,
    pub status_message: std::sync::Arc<std::sync::Mutex<String>>,
}

impl App {
    pub fn new() -> Self {
        let mut status_message = String::new();

        let yt_dlp_path = backend::os_util::OSUtil::get_yt_dlp_path();
        let ffmpeg_path = backend::os_util::OSUtil::get_ffmpeg_path();

        if !yt_dlp_path.exists() {
            status_message.push_str(&format!(
                "Error: yt-dlp executable not found at {:?}\n",
                yt_dlp_path
            ));
        }

        if !ffmpeg_path.exists() {
            status_message.push_str(&format!(
                "Error: ffmpeg executable not found at {:?}\n",
                ffmpeg_path
            ));
        }

        if status_message.is_empty() {
            status_message = "All dependencies are available.".to_string();
        }

        Self {
            input_url: String::new(),
            status_message: std::sync::Arc::new(std::sync::Mutex::new(status_message)),
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
                    if let Ok(mut status) = self.status_message.lock() {
                        *status = "Please enter a valid URL.".to_string();
                    }
                } else {
                    if let Ok(mut status) = self.status_message.lock() {
                        *status = "Downloading...".to_string();
                    }

                    let url = self.input_url.clone();
                    let music_dir = dirs::audio_dir().unwrap_or_else(|| PathBuf::from("output"));
                    let output_path = music_dir.join("output.mp3");
                    let status_message = std::sync::Arc::new(std::sync::Mutex::new(self.status_message.clone()));
                    let ctx_clone = ctx.clone();

                    let status_message_clone = status_message.clone();
                    std::thread::spawn(move || {
                        let convert_mp3 = ConvertMp3::new(url, output_path.to_string_lossy().to_string());
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
                        let mut status = status_message_clone.lock().unwrap();
                        if let Ok(mut status_guard) = status.lock() {
                            *status_guard = new_status_message;
                        }

                        ctx_clone.request_repaint();
                    });
                }
            }

            if let Ok(status) = self.status_message.lock() {
                ui.label(status.clone());
            }
        });
    }
}
