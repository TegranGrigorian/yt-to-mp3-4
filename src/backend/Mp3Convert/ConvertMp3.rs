use egui::epaint::tessellator::path;

use crate::backend::multithread_utils;
use crate::backend::os_util::OSUtil;
use crate::backend::rename_files::rename_file_to_video_title; // Import the rename function
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use filetime::{FileTime, set_file_mtime}; // Add this crate to handle file timestamps
use std::io; // Import for error handling

pub struct ConvertMp3 {
    input_file: String,
    output_file: PathBuf, // Changed to PathBuf for better path handling
}

impl ConvertMp3 {
    pub fn new(input_file: String, output_file: PathBuf) -> Self {
        ConvertMp3 {
            input_file,
            output_file,
        }
    }

    pub fn convert(&self) -> Result<(), String> {
        println!("Starting conversion for URL: {}", self.input_file);

        let yt_dlp_path = OSUtil::get_yt_dlp_path();
        let ffmpeg_path = OSUtil::get_ffmpeg_path();

        if !ffmpeg_path.exists() {
            eprintln!("Error: ffmpeg executable not found at {}", ffmpeg_path.display());
            return Err("ffmpeg executable not found".to_string());
        }

        // Ensure the output path includes a valid filename
        let output_file_path = if self.output_file.is_dir() {
            self.output_file.join("output.mp3") // Default to "output.mp3" if only a directory is provided
        } else {
            self.output_file.clone()
        };

        println!("Executing yt-dlp command...");
        let output = std::process::Command::new(yt_dlp_path)
            .env("FFMPEG", ffmpeg_path)
            .arg("-o")
            .arg(output_file_path.to_str().unwrap()) // Use the updated output path
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--audio-quality")
            .arg("0")
            .arg("--concurrent-fragments")
            .arg("24")
            .arg("--extractor-args")
            .arg("youtube:player_client=web")
            .arg("--postprocessor-args")
            .arg(format!("ffmpeg:-threads {}", multithread_utils::MultiThreadUtils::get_num_cpus() - 1))
            .arg(&self.input_file)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                println!("yt-dlp executed successfully.");
                println!("Output file path: {}", output_file_path.display());

                // Update the file's modification time to the current date
                if output_file_path.exists() {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    let file_time = FileTime::from_unix_time(now.as_secs() as i64, now.subsec_nanos());
                    if let Err(e) = set_file_mtime(&output_file_path, file_time) {
                        eprintln!("Failed to update file modification time: {}", e);
                    } else {
                        println!("File modification time updated to the current date.");
                    }

                    // Delegate renaming to the rename_files module
                    if let Err(e) = rename_file_to_video_title(&output_file_path, &self.input_file) {
                        eprintln!("Failed to rename file: {}", e);
                    }
                } else {
                    eprintln!("Output file does not exist.");
                }

                // Open the file's parent directory
                if let Some(parent_dir) = output_file_path.parent() {
                    println!("Opening file explorer at: {}", parent_dir.display());
                    let command_result = if cfg!(target_os = "windows") {
                        std::process::Command::new("explorer").arg(parent_dir).status()
                    } else if cfg!(target_os = "macos") {
                        std::process::Command::new("open").arg(parent_dir).status()
                    } else if cfg!(target_os = "linux") {
                        std::process::Command::new("xdg-open").arg(parent_dir).status()
                    } else {
                        Err(io::Error::new(io::ErrorKind::Other, "Unsupported OS"))
                    };

                    if let Err(e) = command_result {
                        eprintln!("Failed to open file explorer: {}", e);
                    }
                } else {
                    eprintln!("Parent directory could not be determined.");
                }

                Ok(())
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                let error_message = format!("yt-dlp failed with error: {}", stderr);
                eprintln!("{}", error_message);
                Err(error_message)
            }
            Err(e) => {
                let error_message = format!("Failed to execute yt-dlp: {}", e);
                eprintln!("{}", error_message);
                Err(error_message)
            }
        }
    }

    fn get_video_title(&self) -> Option<String> {
        // Use yt-dlp to extract the video title
        let yt_dlp_path = OSUtil::get_yt_dlp_path();
        let output = std::process::Command::new(yt_dlp_path)
            .arg("-e") // Extract title only
            .arg(&self.input_file)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Some(title)
            }
            _ => None,
        }
    }
}