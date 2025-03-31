use std::fs;
use std::path::PathBuf;

pub struct OSUtil;

impl OSUtil {
    //get ytdlp for os
    pub fn get_yt_dlp_path() -> PathBuf {
        let path = if cfg!(target_os = "windows") {
            PathBuf::from("./bin/windows/yt-dlp.exe")
        } else {
            PathBuf::from("./bin/linux/yt-dlp")
        };

        let absolute_path = std::fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        println!("Resolved yt-dlp path: {:?}", absolute_path);

        if !absolute_path.exists() {
            eprintln!("Error: yt-dlp executable not found at {:?}", absolute_path);
        }

        absolute_path
    }

    //get ffmpeg for os
    pub fn get_ffmpeg_path() -> PathBuf {
        if cfg!(target_os = "windows") {
            PathBuf::from("./bin/windows/ffmpeg.exe")
        } else {
            PathBuf::from("./bin/linux/ffmpeg")
        }
    }

    //determine default output folder based on the OS and format
    pub fn get_output_folder(format: &str) -> PathBuf {
        if cfg!(target_os = "windows") {
            match format {
                "mp3" => {
                    if let Some(music_folder) = dirs::audio_dir() {
                        let mut music_folder = music_folder;
                        music_folder.push("yt-to-mp3-mp4");
                        fs::create_dir_all(&music_folder).expect("Failed to create yt-to-mp3-mp4 folder");
                        music_folder
                    } else {
                        let mut fallback_folder = std::env::current_dir().expect("Failed to get current directory");
                        fallback_folder.push("downloads");
                        fs::create_dir_all(&fallback_folder).expect("Failed to create downloads folder");
                        fallback_folder
                    }
                }
                "mp4" => {
                    if let Some(video_folder) = dirs::video_dir() {
                        let mut video_folder = video_folder;
                        video_folder.push("yt-to-mp3-mp4");
                        fs::create_dir_all(&video_folder).expect("Failed to create yt-to-mp3-mp4 folder");
                        video_folder
                    } else {
                        let mut fallback_folder = std::env::current_dir().expect("Failed to get current directory");
                        fallback_folder.push("downloads");
                        fs::create_dir_all(&fallback_folder).expect("Failed to create downloads folder");
                        fallback_folder
                    }
                }
                _ => panic!("Unsupported format: {}", format),
            }
        } else {
            // Use a "downloads" folder in the project directory on Linux
            let mut downloads_folder = std::env::current_dir().expect("Failed to get current directory");
            downloads_folder.push("downloads");
            fs::create_dir_all(&downloads_folder).expect("Failed to create downloads folder");
            downloads_folder
        }
    }
}