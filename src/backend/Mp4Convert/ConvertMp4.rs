use crate::backend::os_util::OSUtil; // Corrected the import to match the module name
use std::process::Command;

pub struct ConvertMp4 {
    input_file: String,
    output_file: String,
}

impl ConvertMp4 {
    pub fn new(input_file: String, output_file: String) -> Self {
        ConvertMp4 {
            input_file,
            output_file,
        }
    }

    pub async fn convert(&self) {
        let yt_dlp_path = OSUtil::get_yt_dlp_path();
        let ffmpeg_path = OSUtil::get_ffmpeg_path();
        let output_folder = OSUtil::get_output_folder();

        if !ffmpeg_path.exists() {
            eprintln!("Error: ffmpeg executable not found at {}", ffmpeg_path.display());
            return;
        }

        let output_file_path = output_folder.join("%(title)s.mp4");

        let output = Command::new(yt_dlp_path)
            .env("FFMPEG", ffmpeg_path)
            .arg("-o")
            .arg(output_file_path.to_str().unwrap()) // Specify the output file
            .arg("-f")
            .arg("bestvideo+bestaudio[ext=mp4]/mp4") // Download best video and audio, merge into MP4
            .arg("--concurrent-fragments")
            .arg("4") // Enable parallel fragment downloads
            .arg(&self.input_file) // Input video URL
            .output();

        match output {
            Ok(output) if output.status.success() => {
                println!(
                    "Video downloaded successfully to: {}",
                    output_file_path.to_str().unwrap()
                );
            }
            Ok(output) => {
                eprintln!(
                    "Failed to download video: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(e) => {
                eprintln!("Error executing yt-dlp command: {}", e);
            }
        }
    }
}