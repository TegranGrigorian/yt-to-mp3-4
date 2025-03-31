use std::path::PathBuf;
use crate::backend::convert::mp3::ConvertMp3;

pub struct BatchConvertMp3;

impl BatchConvertMp3 {
    pub fn convert_batch(playlist_name: &str, songs: Vec<String>, output_dir: &PathBuf) {
        println!("Starting batch MP3 conversion for playlist: {}", playlist_name);

        let playlist_folder = output_dir.join(playlist_name);
        println!("Creating folder at: {:?}", playlist_folder);
        if let Err(e) = std::fs::create_dir_all(&playlist_folder) {
            eprintln!("Failed to create playlist folder: {}", e);
            return;
        }

        for (index, song_url) in songs.iter().enumerate() {
            println!("Processing song {}/{}: {}", index + 1, songs.len(), song_url);
            let output_file = playlist_folder.join(format!("{:02}_song.mp3", index + 1));
            println!("Saving song to: {:?}", output_file);

            let convert_mp3 = ConvertMp3::new(song_url.clone(), output_file.to_string_lossy().to_string());

            match convert_mp3.convert() {
                Ok(_) => println!("Successfully downloaded song {}/{}", index + 1, songs.len()),
                Err(err) => eprintln!("Failed to download song {}: {}", index + 1, err),
            }
        }

        println!("Batch MP3 conversion completed for playlist: {}", playlist_name);
    }
}