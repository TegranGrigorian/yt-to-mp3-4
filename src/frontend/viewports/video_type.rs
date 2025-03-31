use eframe::egui;

pub fn video_type_viewport(
    ui: &mut egui::Ui,
    video_type: &mut String,
    on_next: &mut dyn FnMut(String),
    on_back: &mut dyn FnMut(),
) {
    ui.heading("Select Video Type");

    if ui.button("Single Video").clicked() {
        on_next("Single".to_string());
    }

    if ui.button("Batch (Album/Playlist)").clicked() {
        on_next("Batch".to_string());
    }

    if ui.button("Go back").clicked() {
        on_back();
    }
}