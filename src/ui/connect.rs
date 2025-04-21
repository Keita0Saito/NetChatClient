use crate::app::ChatApp;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, app: &mut ChatApp) {
    ui.vertical_centered(|ui| {
        ui.heading("Connect to Server");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("IP:");
            ui.text_edit_singleline(&mut app.ip);
        });

        ui.horizontal(|ui| {
            ui.label("Port:");
            ui.text_edit_singleline(&mut app.port);
        });

        ui.add_space(10.0);

        if ui.button("Connect").clicked() {
            app.try_connect();
        }

        if let Some(err) = &app.error_message {
            ui.colored_label(egui::Color32::RED, err);
        }
    });
}
