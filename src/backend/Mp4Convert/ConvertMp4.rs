//we was gonna use rust tube but we are using yt dlp in command line

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
        let output = Command::new("yt-dlp")
            .arg("-o")
            .arg(&self.output_file)
            .arg("-f")
            .arg("bestvideo+bestaudio[ext=mp4]/mp4") //this gets best video but can take a while. The gui will give an option
            .arg(&self.input_file)
            .output()
            .expect("Error exectuing yt-dlp command, check if installtion is present on local");

        if output.status.success() {
            println!("Video downloaded successfully to: {}", self.output_file);
        } else {
            eprintln!(
                "Failed to download video: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}

//arg list for just mp4
// .arg("-o")
//             .arg(&self.output_file) // Specify the output file
//             .arg("-f")
//             .arg("mp4") // Enforce MP4 format
//             .arg(&self.input_file)  // Specify the video URL
//             .output()
//             .expect("Failed to execute yt-dlp");
