use eframe::egui;
use std::path::PathBuf;
use rfd;
pub fn format_and_directory_selection(
    ui: &mut egui::Ui,
    format: &mut String,
    output_dir: &mut PathBuf,
    status_message: &str,
    on_next: &mut dyn FnMut(),
) {
    ui.heading("Select Format and Output Directory");

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

    // Button to select the output directory
    ui.horizontal(|ui| {
        ui.label("Output Directory, if none is selected, yt-to-mp3-4 folder in user's music folder:");
        if ui.button("Choose Directory").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                *output_dir = path;
            }
        }
        if output_dir.as_os_str().is_empty() {
            if let Some(music_dir) = dirs::audio_dir() {
                *output_dir = music_dir.join("yt-to-mp3-4");
            }
        }
        ui.label(output_dir.display().to_string());
    });

    // Display dependency status
    ui.label(status_message);

    // Next button
    if ui.button("Next").clicked() {
        on_next();
    }
}