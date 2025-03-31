use std::path::PathBuf;
use crate::backend::Mp3Convert::ConvertMp3;

pub struct BatchConvertMp3;

impl BatchConvertMp3 {
    pub fn convert_batch(playlist_name: &str, songs: Vec<String>, output_dir: &PathBuf) {
        let playlist_folder = output_dir.join(playlist_name);
        if let Err(e) = std::fs::create_dir_all(&playlist_folder) {
            eprintln!("Failed to create playlist folder: {}", e);
            return;
        }

        for (index, song_url) in songs.iter().enumerate() {
            let output_file = playlist_folder.join(format!("{:02}_song.mp3", index + 1));
            let convert_mp3 = ConvertMp3::new(song_url.clone(), output_file.to_string_lossy().to_string());

            match convert_mp3.convert() {
                Ok(_) => println!("Successfully downloaded song {}/{}", index + 1, songs.len()),
                Err(err) => eprintln!("Failed to download song {}: {}", index + 1, err),
            }
        }
    }
}