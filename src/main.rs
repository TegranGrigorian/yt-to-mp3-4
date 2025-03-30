pub mod backend {
    pub mod initrusttube;
    pub mod UserInput;
    pub mod Mp4Convert;
}
pub mod CommandLineTest;
use tokio;
fn main() {
    // CommandLineTest::test();
    let user_input = backend::UserInput::UserInput::read_from_console("Enter a video URL".to_string());
    
    // // Example usage of the InitRustTube module
    // let init_tube = backend::initrusttube::InitRustTube::new(Some(user_input.clone()));
    // init_tube.initialize();
    // // You can add more logic here to handle the application flow
    // // For example, you might want to create an instance of ConvertMp4 and perform the conversion
    // // Example usage of the ConvertMp4 module
    //     // Clone user_input again before passing it to ConvertMp4
    // let convert_mp4 = backend::Mp4Convert::ConvertMp4::ConvertMp4::new(
    //     user_input.clone(), // Clone here to avoid move
    //     "output.mp4".to_string() 
    // );
    // //lets use the convert method to perform the conversion
    // // Note: The convert method is async, so you would typically call it within an async context
    // tokio::runtime::Builder::new_current_thread()
    //     .enable_all()
    //     .build()
    //     .unwrap()
    //     .block_on(async {
    //     // Call the convert method on the ConvertMp4 instance
    //     convert_mp4.convert().await; // Await the conversion process
    // });
    let convert_mp4 = backend::Mp4Convert::ConvertMp4::ConvertMp4::new(
        user_input.clone(), // Video URL
        "output.mp4".to_string(), // Output file path
    );

    // Use tokio runtime to call the async convert method
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            convert_mp4.convert().await;
        });
}
