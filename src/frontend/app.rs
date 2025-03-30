use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::frontend::screens::main_screen::main_window;
use crate::backend;

use super::screens::single_screen::SingleScreen; // Ensure this module is available for checking dependencies
pub struct App {
    pub input_url: String,
    pub status_message: Arc<Mutex<String>>,
    pub format: String,       // Selected format (MP3 or MP4)
    pub output_dir: PathBuf,  // Selected output directory
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
            status_message: Arc::new(Mutex::new(status_message)),
            format: "MP3".to_string(), // Default format
            output_dir: PathBuf::new(), // Default to empty path
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut status_message = self.status_message.lock().unwrap();

            // Pass the new fields to the main_window function
            main_window(
                ui,
                &mut self.input_url,
                &mut status_message,
                &mut self.format,
                &mut self.output_dir,
            );
        });
    }
}