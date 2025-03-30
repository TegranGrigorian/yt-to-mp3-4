use crate::backend::os_util::OSUtil;
use std::process::Command;

pub struct ConvertMp3 {
    input_file: String,
    output_file: String, //bruh rust compiler this aint read we determined it, eventually the gui can do this too
}

impl ConvertMp3 {
    pub fn new(input_file: String, output_file: String) -> Self {
        ConvertMp3 {
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

        let output_file_path = output_folder.join("%(title)s.mp3");

        let output = Command::new(yt_dlp_path)
            .env("FFMPEG", ffmpeg_path)
            .arg("-o")
            .arg(output_file_path.to_str().unwrap())
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--audio-quality")
            .arg("0")
            .arg("--concurrent-fragments")
            .arg("4")
            .arg(&self.input_file)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                println!(
                    "Audio downloaded successfully to: {}",
                    output_file_path.to_str().unwrap()
                );
            }
            Ok(output) => {
                eprintln!(
                    "Failed to download audio: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(e) => {
                eprintln!("Error executing yt-dlp command: {}", e);
            }
        }
    }
}