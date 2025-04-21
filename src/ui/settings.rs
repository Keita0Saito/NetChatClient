use eframe::egui;

pub fn show_window(ctx: &egui::Context) {
    egui::Window::new("Settings")
        .collapsible(false)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Settings");
            ui.separator();
            ui.label("Here you can adjust your settings!");

            if ui.button("Close").clicked() {
                // Закрытие окна
            }
        });
}
