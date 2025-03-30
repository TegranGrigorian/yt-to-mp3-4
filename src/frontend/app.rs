use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use crate::frontend::app_state::AppState;
use crate::frontend::screens::{main_screen, settings_screen, download_screen, download_logic};
use crate::backend;

use super::screens::video_type;
pub struct App {
    pub input_url: String,
    pub status_message: Arc<Mutex<String>>,
    pub format: String,       // Selected format (MP3 or MP4)
    pub output_dir: PathBuf,  // Selected output directory
    pub state: AppState,      // Current application state
    pub video_type: String,   // "Single" or "Batch"
    pub download_complete: bool, // Flag to track if the download is complete
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
            state: AppState::FormatAndDirectorySelection, // Start with the first screen
            video_type: String::new(), // Will be "Single" or "Batch"
            download_complete: false, // Initially, the download is not complete
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                AppState::FormatAndDirectorySelection => {
                    main_screen::format_and_directory_selection(
                        ui,
                        &mut self.format,
                        &mut self.output_dir,
                        &self.status_message.lock().unwrap(),
                        &mut || self.state = AppState::VideoTypeSelection,
                    );
                }
                AppState::VideoTypeSelection => {
                    video_type::video_type_selection(
                        ui,
                        &mut self.video_type,
                        &mut || self.state = AppState::DownloadScreen,
                    );
                }
                AppState::DownloadScreen => {
                    // Clone the variables before calling `download_screen`
                    let mut input_url = self.input_url.clone(); // Use a local variable for input_url
                    let format = self.format.clone();
                    let video_type = self.video_type.clone();
                    let status_message = Arc::clone(&self.status_message);
                    let ctx = ctx.clone(); // Clone the egui context
                
                    // Define the "Download" button callback
                    let mut on_download = {
                        let input_url = input_url.clone();
                        let format = format.clone();
                        let video_type = video_type.clone();
                        let status_message = Arc::clone(&status_message);
                        let ctx = ctx.clone();
                
                        move || {
                            // Call the `handle_download` function
                            download_logic::handle_download(
                                input_url.clone(),
                                format.clone(),
                                video_type.clone(),
                                Arc::clone(&status_message),
                                ctx.clone(),
                            );
                        }
                    };
                
                    // Define the "Convert Again" button callback
                    let mut on_convert_again = {
                        let status_message = Arc::clone(&self.status_message);
                        let state = &mut self.state;
                        let input_url = &mut self.input_url;
                        move || {
                            *state = AppState::FormatAndDirectorySelection;
                            input_url.clear(); // Clear the text box
                            if let Ok(mut status) = status_message.lock() {
                                *status = "All dependencies are available.".to_string(); // Reset the label
                            }
                        }
                    };
                
                    // Call the `download_screen` function
                    download_screen::download_screen(
                        ui,
                        &mut input_url, // Pass the local variable for editing
                        status_message, // Pass the cloned status message
                        &format,
                        &video_type,
                        &mut on_download,
                        &mut on_convert_again,
                    );
                
                    // Update self.input_url after `download_screen` is done
                    self.input_url = input_url;
                }
                AppState::SettingsScreen => {
                    settings_screen::settings_screen(ui, &mut || self.state = AppState::FormatAndDirectorySelection);
                }
            }
        });
    }
}