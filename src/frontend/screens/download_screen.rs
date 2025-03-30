pub fn download_screen(
    ui: &mut egui::Ui,
    input_url: &mut String,
    status_message: &mut String,
    format: &String,
    video_type: &String,
    on_download: &mut dyn FnMut(),
    on_convert_again: &mut dyn FnMut(), // Callback for "Convert Again"
) {
    ui.heading(format!("Download {} as {}", video_type, format));

    // Input field for the YouTube URL
    ui.horizontal(|ui| {
        ui.label("YouTube URL:");
        ui.text_edit_singleline(input_url);
    });

    // Download button
    if ui.button("Download").clicked() {
        on_download();
    }

    // Display the status message
    ui.label(status_message.clone());

    // Conditionally show the "Convert Again" button
    if status_message.contains("Download complete!") {
        if ui.button("Convert Again").clicked() {
            on_convert_again();
        }
    }
}