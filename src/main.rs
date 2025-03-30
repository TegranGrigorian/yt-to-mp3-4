pub mod backend {
    // pub mod initrusttube;
    pub mod UserInput;
    pub mod Mp4Convert;
    pub mod Mp3Convert;
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
    // Create a Tokio runtime to run the async function 
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            convert_mp3.convert().await; // Call the async convert method
        });
}
