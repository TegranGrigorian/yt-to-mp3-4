use crate::backend::multithread_utils;
use crate::backend::os_util::OSUtil;
use std::process::Output;

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

    pub fn convert(&self) -> Result<(), String> {
        println!("Starting conversion for URL: {}", self.input_file);

        let yt_dlp_path = OSUtil::get_yt_dlp_path();
        let ffmpeg_path = OSUtil::get_ffmpeg_path();
        let output_folder = OSUtil::get_output_folder("mp4");
        if !output_folder.exists() {
            eprintln!("Error: Output folder does not exist: {}", output_folder.display());
            return Err("Output folder does not exist".to_string());
        }

        let thread_arg = format!("ffmpeg:-threads {}", multithread_utils::MultiThreadUtils::get_num_cpus() - 1);
        let output_file_path = output_folder.join("%(title)s.mp4");

        println!("Executing yt-dlp command...");
        let output = std::process::Command::new(yt_dlp_path)
            .env("FFMPEG", ffmpeg_path)
            .arg("-o")
            .arg(output_file_path.to_str().unwrap())
            .arg("-f")
            .arg("bestvideo+bestaudio[ext=mp4]/mp4")
            .arg("--concurrent-fragments")
            .arg("24")
            .arg("--postprocessor-args")
            .arg(&thread_arg)
            .arg(&self.input_file)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                println!("yt-dlp executed successfully.{}", output_file_path.display());
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

    fn handle_output(&self, output: Output) -> Result<(), String> {
        if output.status.success() {
            println!("yt-dlp executed successfully.");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let error_message = format!("yt-dlp failed with error: {}", stderr);
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}