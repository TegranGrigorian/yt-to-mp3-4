use std::path::PathBuf;
use std::fs;

pub fn download_playlist_in_order(link: &str, format: &str, output_dir: &PathBuf, status_message: &mut String) -> usize {
    println!("Starting download for playlist: {}", link);

    // Simulate fetching playlist details
    let total_songs = 10; // Example: Assume 10 songs in the playlist
    let playlist_name = "Example Playlist"; // Simulate fetching the playlist name
    println!("Playlist name: {}", playlist_name);

    // Create a folder for the playlist/album
    let playlist_folder = output_dir.join(playlist_name);
    println!("Creating folder at: {:?}", playlist_folder);
    if let Err(e) = fs::create_dir_all(&playlist_folder) {
        eprintln!("Failed to create playlist folder: {}", e);
        return 0;
    }

    for i in 1..=total_songs {
        *status_message = format!("Downloading song {}/{}...", i, total_songs);
        println!("Downloading song {}/{}...", i, total_songs);

        // Simulate download and tagging process
        let output_file = playlist_folder.join(format!("{:02}_song{}.{}", i, i, format.to_lowercase()));
        println!("Saving song to: {:?}", output_file);
        if let Err(e) = fs::write(&output_file, b"Dummy content") {
            eprintln!("Failed to write file for song {}: {}", i, e);
            continue;
        }

        println!("Completed song {}/{}", i, total_songs);
    }

    println!("Download completed for playlist: {}", playlist_name);
    total_songs
}