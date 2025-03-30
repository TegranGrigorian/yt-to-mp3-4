use eframe::egui;

pub fn video_type_selection(
    ui: &mut egui::Ui,
    video_type: &mut String,
    on_next: &mut dyn FnMut(),
) {
    ui.heading("Select Video Type");

    if ui.button("Single Video").clicked() {
        *video_type = "Single".to_string();
        on_next();
    }

    if ui.button("Batch (Album/Playlist)").clicked() {
        *video_type = "Batch".to_string();
        on_next();
    }
}