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
    
    //current implementaiton isnt the best, but it works for now. This will be improved with viewports in the future.
    if ui.button("Go Back").clicked() {
        //restart exe, place holder until viewports are integrated
        std::process::Command::new(std::env::current_exe().unwrap()) // Relaunch the current executable
        .spawn()
        .expect("Failed to relaunch application");
        std::process::exit(0); // Exit the current process
    }
}