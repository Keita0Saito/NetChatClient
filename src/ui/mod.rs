use eframe::egui;
use crate::app::{AppState, ChatApp};

pub fn show_main_panel(app: &mut ChatApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        match app.state {
            AppState::Connect => show_connect_screen(ui, app),
            AppState::Chat => show_chat_screen(ui, ctx, app),
        }
    });
}

fn show_connect_screen(ui: &mut egui::Ui, app: &mut ChatApp) {
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

fn show_chat_screen(ui: &mut egui::Ui, ctx: &egui::Context, app: &mut ChatApp) {
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
        show_chat_messages(ui, app);
        show_message_input(ui, ctx, app);
    });

    if show_settings {
        show_settings_window(ctx);
    }
}

fn show_chat_messages(ui: &mut egui::Ui, app: &mut ChatApp) {
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

fn show_message_input(ui: &mut egui::Ui, ctx: &egui::Context, app: &mut ChatApp) {
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

fn show_settings_window(ctx: &egui::Context) {
    egui::Window::new("Settings")
        .collapsible(false)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Settings");
            ui.separator();
            ui.label("Here you can adjust your settings!");
            
            if ui.button("Close").clicked() {
                // Window will close automatically
            }
        });
}
