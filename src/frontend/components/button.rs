use eframe::egui;

pub fn custom_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.button(label)
}