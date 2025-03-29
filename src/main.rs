mod app;
mod ui;
mod net;

use eframe::NativeOptions;
use app::ChatApp;

fn main() {
    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "NetChat Client",
        options,
        Box::new(|_cc| Ok(Box::new(ChatApp::new())))
    );
}
