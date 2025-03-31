use crate::backend::multithread_utils;
use crate::backend::os_util::OSUtil;
// use crate::backend::multithread_utils::MultiThreadUtils; // for getting the number of CPU cores
// use std::process::Command;
use std::process::Output;

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

    pub fn convert(&self) -> Result<(), String> {
        println!("Starting conversion for URL: {}", self.input_file);

        let yt_dlp_path = OSUtil::get_yt_dlp_path();
        let ffmpeg_path = OSUtil::get_ffmpeg_path();
        let output_folder = OSUtil::get_output_folder("mp3");

        if !ffmpeg_path.exists() {
            eprintln!("Error: ffmpeg executable not found at {}", ffmpeg_path.display());
            return Err("ffmpeg executable not found".to_string());
        }

        let thread_arg = format!("ffmpeg:-threads {}", multithread_utils::MultiThreadUtils::get_num_cpus() - 1);
        let output_file_path = output_folder.join("%(title)s.mp3");

        println!("Executing yt-dlp command...");
        let output = std::process::Command::new(yt_dlp_path)
            .env("FFMPEG", ffmpeg_path)
            .arg("-o")
            .arg(output_file_path.to_str().unwrap())
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
            .arg(&thread_arg)
            .arg(&self.input_file)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                println!("yt-dlp executed successfully.");
                println!("Output file path: {}", output_file_path.display());

                // Ensure the file exists before attempting to open the directory
                // println!(output_file_path.display()); 
                println!("Path exists");
                if let Some(parent_dir) = output_file_path.parent() {
                    println!("Opening file explorer at: {}", parent_dir.display()); // Debug
                    let command_result = if cfg!(target_os = "windows") {
                        std::process::Command::new("explorer" ).arg(parent_dir).status()
                    } else if cfg!(target_os = "macos") {
                        std::process::Command::new("open").arg(parent_dir).status()
                    } else if cfg!(target_os = "linux") {
                        std::process::Command::new("xdg-open").arg(parent_dir).status()
                    } else {
                        Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported OS"))
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

    // fn handle_output(&self, output: Output) -> Result<(), String> {
    //     if output.status.success() {
    //         println!("yt-dlp executed successfully.");
    //         Ok(())
    //     } else {
    //         let stderr = String::from_utf8_lossy(&output.stderr);
    //         let error_message = format!("yt-dlp failed with error: {}", stderr);
    //         eprintln!("{}", error_message);
    //         Err(error_message)
    //     }
    // }
}