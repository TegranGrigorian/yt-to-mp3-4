pub fn test() {
    let video_url = "https://www.youtube.com/watch?v=wMH0e8kIZtE"; // Welcome to Heartbreak by Kanye West
    let output = std::process::Command::new("yt-dlp")
        .arg("-o")
        .arg("output.mp4")
        .arg(&video_url)
        .output()
        .expect("Failed to execute yt-dlp");

    if output.status.success() {
        println!("Video downloaded successfully!");
    } else {
        eprintln!(
            "Failed to download video: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}