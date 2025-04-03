// use std::option;

// use eframe::NativeOptions;
// use frontend::app::App;
pub mod backend {
    // pub mod initrusttube;
    pub mod UserInput;
    pub mod Mp4Convert;
    pub mod Mp3Convert;
    pub mod multithread_utils;
    pub mod batch_download;
    pub mod os_util; // Corrected the module name to os_util to match the file name
    pub mod rename_files;
}
pub mod frontend {
    pub mod components; // This module contains reusable components like buttons
    pub mod screens; // This module contains different screens for the GUI
    pub mod style; // This module contains the main window and other windows for the GUI
    pub mod app;
    pub mod app_state; // This module contains the application state, used for managing the state of the GUI
}
// pub mod CommandLineTest;
// use tokio;

// async fn test_tokio_sleep() {
//     println!("Before sleep...");
//     tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//     println!("After sleep...");
// }
fn main() {
    // Set up a panic hook to catch any panics and log them
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Panic occurred: {:?}", panic_info);
    }));

    // Create the Tokio runtime
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    // Run the GUI application within the Tokio runtime
    runtime.block_on(async_main());
}

async fn async_main() {
    let options = eframe::NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "YouTube to MP3 Converter",
        options,
        Box::new(|_cc| Ok(Box::new(crate::frontend::app::App::new()))),
    ) {
        eprintln!("Error running the application: {}", e);
    }
}
// fn check_dependencies() -> bool {
//     let yt_dlp_path = backend::os_util::OSUtil::get_yt_dlp_path();
//     let ffmpeg_path = backend::os_util::OSUtil::get_ffmpeg_path();

//     let yt_dlp_exists = yt_dlp_path.exists();
//     let ffmpeg_exists = ffmpeg_path.exists();

//     if !yt_dlp_exists {
//         eprintln!("Error: yt-dlp executable not found at {:?}", yt_dlp_path);
//     }

//     if !ffmpeg_exists {
//         eprintln!("Error: ffmpeg executable not found at {:?}", ffmpeg_path);
//     }

//     yt_dlp_exists && ffmpeg_exists
// }