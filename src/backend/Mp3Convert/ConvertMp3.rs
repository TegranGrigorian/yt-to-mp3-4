use std::process;
use std::fs;
use std::io::{self, Write}; // Import io for flushing stdout and reading input
use std::path::PathBuf;

pub struct ConvertMp3 {
    input_file: String,
    output_file: String,
}

impl ConvertMp3 {
    pub fn new(input_file: String, output_file: String) -> Self {
        ConvertMp3 {
            input_file,
            output_file,
        }
    }

    pub async fn convert(&self) {
        // If Windows
        #[cfg(target_os = "windows")]
        let yt_dlp_path = "./bin/windows/yt-dlp.exe";
        #[cfg(target_os = "windows")]
        let ffmpeg_path = "./bin/windows/ffmpeg.exe";

        // If Linux
        #[cfg(target_os = "linux")]
        let yt_dlp_path = "./bin/linux/yt-dlp";
        #[cfg(target_os = "linux")]
        let ffmpeg_path = "./bin/linux/ffmpeg";

        // Validate ffmpeg_path
        if fs::metadata(ffmpeg_path).is_err() {
            eprintln!("Error: ffmpeg executable not found at {}", ffmpeg_path);
            self.wait_for_exit();
            return;
        }

        // Get the user's Music folder and create the yt-to-mp3-mp4 subfolder
        let mut music_folder = dirs::audio_dir().unwrap_or_else(|| {
            eprintln!("Error: Could not locate the Music folder.");
            self.wait_for_exit();
            std::process::exit(1);
        });
        music_folder.push("yt-to-mp3-mp4");
        fs::create_dir_all(&music_folder).expect("Failed to create yt-to-mp3-mp4 folder");

        // Set the output file path with the YouTube video title
        let output_file_path = music_folder.join("%(title)s.mp3");

        // Run yt-dlp with the appropriate arguments
        let output = std::process::Command::new(yt_dlp_path)
            .env("FFMPEG", ffmpeg_path)
            .arg("-o")
            .arg(output_file_path.to_str().unwrap()) // Use the full path for the output file
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("mp3")
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

        // Wait for user input before exiting
        self.wait_for_exit();
    }

    fn wait_for_exit(&self) {
        print!("Press Enter to exit...");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut String::new());
    }
}