use std::option;

use eframe::NativeOptions;
use frontend::app::App;
pub mod backend {
    // pub mod initrusttube;
    pub mod UserInput;
    pub mod Mp4Convert;
    pub mod Mp3Convert;
    pub mod multithread_utils;
    pub mod os_util; // Corrected the module name to os_util to match the file name
}
pub mod frontend {
    pub mod components; // This module contains reusable components like buttons
    pub mod screens; // This module contains different screens for the GUI
    pub mod style; // This module contains the main window and other windows for the GUI
    pub mod app;
    pub mod events;
}
pub mod CommandLineTest;
use tokio;
fn main() {
    // CommandLineTest::test(); //for debugging with cli tool yt-dlp
    // let user_input = backend::UserInput::UserInput::read_from_console("Enter a video URL".to_string());
    // let convert_mp4 = backend::Mp4Convert::ConvertMp4::ConvertMp4::new(
    //     user_input.clone(),
    //     "output.mp4".to_string(),
    // );

    // tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(async {
    //         convert_mp4.convert().await;
    //     });
    let user_input = backend::UserInput::UserInput::read_from_console("Enter a video URL".to_string());
    let convert_mp3 = backend::Mp3Convert::ConvertMp3::ConvertMp3::new(
        user_input.clone(),
        "output.mp3".to_string(), // specify the output file name
    );
    let start_time = std::time::Instant::now();
    // Create a Tokio runtime to run the async function 
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            convert_mp3.convert().await; // Call the async convert method
            let elapsed_time = start_time.elapsed();
            println!("Download completed in {:.2?} seconds", elapsed_time);
        });
    // Start the GUI application
    let options = NativeOptions::default();
    if let Err(e) = eframe::run_native(
        "Youtube to Mp3-4 Converter", 
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))), // Initialize the App struct
    ) {
        eprintln!("Error running the application: {}", e);
    }
}
