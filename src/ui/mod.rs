mod chat;
mod connect;
mod settings;

use crate::app::{AppState, ChatApp};
use eframe::egui;

pub fn show_main_panel(app: &mut ChatApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| match app.state {
        AppState::Connect => connect::show(ui, app),
        AppState::Chat => chat::show(ui, ctx, app),
    });
}
