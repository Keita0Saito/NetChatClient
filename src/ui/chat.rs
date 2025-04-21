use crate::app::ChatApp;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, ctx: &egui::Context, app: &mut ChatApp) {
    let mut show_settings = false;

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.heading("Chat");
            ui.add_space(ui.available_width() - 160.0);

            if ui.button("Settings").clicked() {
                show_settings = !show_settings;
            }

            if ui.button("Back").clicked() {
                app.disconnect();
            }
        });

        ui.separator();
        show_messages(ui, app);
        show_input(ui, ctx, app);
    });

    if show_settings {
        super::settings::show_window(ctx);
    }
}

fn show_messages(ui: &mut egui::Ui, app: &mut ChatApp) {
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .max_height(ui.available_height() - 40.0)
        .show(ui, |ui| {
            for msg in &app.messages {
                ui.label(msg);
            }
        });
}

fn show_input(ui: &mut egui::Ui, ctx: &egui::Context, app: &mut ChatApp) {
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
        ui.separator();
        ui.horizontal(|ui| {
            let text_edit = ui.add(
                egui::TextEdit::singleline(&mut app.message_input)
                    .desired_width(ui.available_width() - 60.0),
            );

            if text_edit.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                app.send_message();
                text_edit.request_focus();
            }

            if ui.button("Send").clicked() {
                app.send_message();
            }
        });
    });
}
