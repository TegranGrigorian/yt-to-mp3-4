use eframe::egui;
use std::path::PathBuf;

pub fn format_and_directory_selection(
    ui: &mut egui::Ui,
    format: &mut String,
    output_dir: &mut PathBuf,
    status_message: &str,
    on_next: &mut dyn FnMut(),
) {
    //this code doesnt do format, that has been moved to the download screen
    ui.heading("Select Format");

    // Dropdown for selecting the format
    ui.horizontal(|ui| {
        ui.label("Select Format:");
        egui::ComboBox::from_label("")
            .selected_text(format.clone())
            .show_ui(ui, |ui| {
                ui.selectable_value(format, "MP3".to_string(), "MP3");
                ui.selectable_value(format, "MP4".to_string(), "MP4");
            });
    });

    // Display the output directory based on the selected format
    ui.horizontal(|ui| {
        ui.label("Default Output Directory:");
        let default_dir = if format == "MP3" {
            dirs::audio_dir().map(|dir| dir.join("yt-to-mp3-4")).unwrap_or_else(|| PathBuf::from("./downloads"))
        } else {
            dirs::video_dir().map(|dir| dir.join("yt-to-mp3-4")).unwrap_or_else(|| PathBuf::from("./downloads"))
        };
        *output_dir = default_dir;
        ui.label(output_dir.display().to_string());
    });

    // Display dependency status
    ui.label(status_message);

    // Next button
    if ui.button("Next").clicked() {
        on_next();
    }
}