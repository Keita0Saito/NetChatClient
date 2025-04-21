mod app;
mod net;
mod ui;

use app::ChatApp;
use eframe::NativeOptions;

fn main() {
    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "NetChat Client",
        options,
        Box::new(|_cc| Ok(Box::new(ChatApp::new()))),
    );
}
