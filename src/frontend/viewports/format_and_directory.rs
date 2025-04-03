use eframe::egui;

//NOTE this code does not do directory changes anymore sorry
pub fn format_and_directory_viewport(
    ui: &mut egui::Ui,
    format: &mut String,
    output_dir: &mut std::path::PathBuf,
    status_message: &str,
    on_next: &mut dyn FnMut(),
) {
    ui.heading("Format");

    ui.label("Select Format:");
    ui.radio_value(format, "MP3".to_string(), "MP3");
    ui.radio_value(format, "MP4".to_string(), "MP4");

    ui.label("Output Directory:");
    if ui.button("Select Directory").clicked() {
        // Logic to select directory
        //open explorer to select directory, this is platform dependent and needs to be implemented in the backend or use a crate
        if let Some(dir) = rfd::FileDialog::new().pick_folder() {
            *output_dir = dir; // Update the output directory
            // Update status message to reflect the new directory
            println!("Selected directory: {:?}", output_dir);
        } else {
            println!("No directory selected.");
        }
    }

    ui.label(status_message);

    if ui.button("Next").clicked() {
        on_next();
    }
}