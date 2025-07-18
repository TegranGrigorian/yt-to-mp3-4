use crate::backend::multithread_utils;
use crate::backend::os_util::OSUtil;
use crate::backend::rename_files::rename_file_to_video_title; // Import the rename function
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use filetime::{FileTime, set_file_mtime}; // Add this crate to handle file timestamps
use std::io; // Import for error handling
use std::fs; // Import for filesystem operations

pub struct ConvertMp4 {
    input_file: String,
    output_file: PathBuf, // Changed to PathBuf for better path handling
}

impl ConvertMp4 {
    pub fn new(input_file: String, output_file: PathBuf) -> Self {
        ConvertMp4 {
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

        // Determine the output template for yt-dlp and expected output path
        let (output_template, expected_output_path) = if self.output_file.is_dir() {
            // If it's a directory, let yt-dlp use default naming in that directory
            let template = self.output_file.join("%(title)s.%(ext)s");
            (template.clone(), template) // We'll need to find the actual file later
        } else if self.output_file.extension().is_none() {
            // If no extension provided, add .mp4 extension for both template and expected path
            let with_extension = PathBuf::from(format!("{}.mp4", self.output_file.display()));
            (self.output_file.clone(), with_extension)
        } else {
            // File already has extension, use as-is
            (self.output_file.clone(), self.output_file.clone())
        };

        println!("Executing yt-dlp command...");
        let output = std::process::Command::new(yt_dlp_path)
            .env("FFMPEG", ffmpeg_path)
            .arg("-o")
            .arg(output_template.to_str().unwrap())
            .arg("-f")
            .arg("bestvideo[ext=mp4][height<=?1080]+bestaudio[ext=m4a]/bestvideo[ext=mp4]+bestaudio[ext=m4a]/bestvideo+bestaudio/best[ext=mp4]/best")
            .arg("--merge-output-format")
            .arg("mp4")
            .arg("--embed-metadata")
            .arg("--concurrent-fragments")
            .arg("24")
            .arg("--extractor-args")
            .arg("youtube:player_client=ios,web")
            .arg("--postprocessor-args")
            .arg(format!("ffmpeg:-threads {} -movflags +faststart -avoid_negative_ts make_zero -fflags +genpts", multithread_utils::MultiThreadUtils::get_num_cpus() - 1))
            .arg("--no-check-certificate")
            .arg("--retries")
            .arg("10")
            .arg("--fragment-retries")
            .arg("10")
            .arg(&self.input_file)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                println!("yt-dlp executed successfully.");
                println!("Expected output file path: {}", expected_output_path.display());

                // Check if the file exists at the expected location
                if expected_output_path.exists() {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    let file_time = FileTime::from_unix_time(now.as_secs() as i64, now.subsec_nanos());
                    if let Err(e) = set_file_mtime(&expected_output_path, file_time) {
                        eprintln!("Failed to update file modification time: {}", e);
                    } else {
                        println!("File modification time updated to the current date.");
                    }

                    // Delegate renaming to the rename_files module
                    if let Err(e) = rename_file_to_video_title(&expected_output_path, &self.input_file) {
                        eprintln!("Failed to rename file: {}", e);
                    }
                } else {
                    eprintln!("Output file does not exist at expected path: {}", expected_output_path.display());
                    eprintln!("yt-dlp might have created the file with a different name.");
                    
                    // If the file doesn't exist at expected path, check the parent directory
                    if let Some(parent_dir) = expected_output_path.parent() {
                        println!("Checking directory: {}", parent_dir.display());
                        if let Ok(entries) = fs::read_dir(parent_dir) {
                            for entry in entries.flatten() {
                                let path = entry.path();
                                if path.extension().map_or(false, |ext| ext == "mp4") {
                                    println!("Found MP4 file: {}", path.display());
                                }
                            }
                        }
                    }
                }

                // Open the file's parent directory
                let parent_dir = if expected_output_path.exists() {
                    expected_output_path.parent()
                } else {
                    // If file doesn't exist at expected path, open the target directory
                    if self.output_file.is_dir() {
                        Some(self.output_file.as_path())
                    } else {
                        self.output_file.parent()
                    }
                };

                if let Some(parent_dir) = parent_dir {
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