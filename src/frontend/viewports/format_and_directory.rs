use eframe::egui;

pub fn format_and_directory_viewport(
    ui: &mut egui::Ui,
    format: &mut String,
    output_dir: &mut std::path::PathBuf,
    status_message: &str,
    on_next: &mut dyn FnMut(),
) {
    ui.heading("Format and Directory Selection");

    ui.label("Select Format:");
    ui.radio_value(format, "MP3".to_string(), "MP3");
    ui.radio_value(format, "MP4".to_string(), "MP4");

    ui.label("Output Directory:");
    if ui.button("Select Directory").clicked() {
        // Logic to select directory
    }

    ui.label(status_message);

    if ui.button("Next").clicked() {
        on_next();
    }
}