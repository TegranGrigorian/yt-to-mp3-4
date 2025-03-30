//convert to mp3 :) main use

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
        let output = std::process::Command::new("yt-dlp")
        .arg("-o")
        .arg(&self.output_file)
        .arg("--extract-audio") 
        .arg("--audio-format")
        .arg("mp3")
        .arg(&self.input_file) 
        .output()
        .expect("Error executing yt-dlp command, check if installation is present on local");

        if output.status.success() {
            println!("Audio downloaded successfully to: {}", self.output_file);
        } else {
            eprintln!(
                "Failed to download audio: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}