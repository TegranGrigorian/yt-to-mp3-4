use eframe::egui;

pub fn settings_screen(ui: &mut egui::Ui, on_return: &mut dyn FnMut()) {
    ui.heading("Settings");

    if ui.button("Save").clicked() {
        // Placeholder: Add save logic here
    }

    if ui.button("Return").clicked() {
        on_return();
    }
}