use std::fs;
use std::io;
use std::path::Path;
use crate::backend::os_util::OSUtil;

/// Renames a file to the YouTube video title.
/// 
/// # Arguments
/// - `file_path`: The current path of the file.
/// - `input_url`: The URL of the YouTube video.
///
/// # Returns
/// - `Ok(())` if the file was renamed successfully.
/// - `Err(String)` if an error occurred.
pub fn rename_file_to_video_title(file_path: &Path, input_url: &str) -> Result<(), String> {
    // Use yt-dlp to extract the video title
    let yt_dlp_path = OSUtil::get_yt_dlp_path();
    let output = std::process::Command::new(yt_dlp_path)
        .arg("-e") // Extract title only
        .arg(input_url)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let video_title = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Sanitize the video title to remove invalid characters
            let sanitized_title = sanitize_filename::sanitize(video_title);

            // Construct the new file path
            let new_file_path = file_path
                .parent()
                .unwrap()
                .join(format!("{}.{}", sanitized_title, file_path.extension().unwrap().to_string_lossy()));

            // Rename the file
            fs::rename(file_path, &new_file_path).map_err(|e| format!("Failed to rename file: {}", e))?;

            println!("File renamed to: {}", new_file_path.display());
            Ok(())
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to extract video title: {}", stderr))
        }
        Err(e) => Err(format!("Failed to execute yt-dlp: {}", e)),
    }
}
