use std::process::Command;

pub struct ConvertMp4 {
    input_file: String,  // This will hold the video URL
    output_file: String, // This will hold the output file path
}

impl ConvertMp4 {
    // Constructor to create a new instance of ConvertMp4
    pub fn new(input_file: String, output_file: String) -> Self {
        ConvertMp4 {
            input_file,
            output_file,
        }
    }

    pub async fn convert(&self) {
        // Use yt-dlp to download the video
        let output = Command::new("yt-dlp")
            .arg("-o")
            .arg(&self.output_file) // Specify the output file
            .arg("-f") // Optional: specify format, e.g., best or mp4
            .arg("bestvideo+bestaudio[ext=mp4]/mp4") // You can specify the format you want to download, e.g., mp4
            .arg(&self.input_file)  // Specify the video URL
            .output()
            .expect("Error exectuing yt-dlp command, check if installtion is present on local"); // Execute the command and capture the output

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