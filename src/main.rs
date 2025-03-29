use eframe::egui;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

#[derive(PartialEq)]
enum AppState {
    Connect,
    Chat,
}

enum ThreadMessage {
    Connected(Result<TcpStream, String>),
    MessageReceived(String),
    Error(String),
}

struct ChatApp {
    state: AppState,
    ip: String,
    port: String,
    error_message: Option<String>,
    message_input: String,
    messages: Vec<String>,
    stream: Option<TcpStream>,
    tx: Sender<ThreadMessage>,
    rx: Receiver<ThreadMessage>,
}

impl ChatApp {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            state: AppState::Connect,
            ip: "127.0.0.1".to_string(),
            port: "8080".to_string(),
            error_message: None,
            message_input: String::new(),
            messages: Vec::new(),
            stream: None,
            tx,
            rx,
        }
    }

    fn try_connect(&mut self) {
        let addr = format!("{}:{}", self.ip, self.port);
        let tx = self.tx.clone();
        thread::spawn(move || match TcpStream::connect(&addr) {
            Ok(stream) => {
                let _ = tx.send(ThreadMessage::Connected(Ok(stream)));
            }
            Err(e) => {
                let _ = tx.send(ThreadMessage::Connected(Err(e.to_string())));
            }
        });
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                ThreadMessage::Connected(Ok(stream)) => {
                    self.stream = Some(stream.try_clone().expect("Failed to clone stream"));
                    self.state = AppState::Chat;
                    let tx = self.tx.clone();
                    let mut stream = self.stream.as_ref().unwrap().try_clone().unwrap();
                    thread::spawn(move || {
                        let mut buf = [0; 1024];
                        loop {
                            match stream.read(&mut buf) {
                                Ok(size) if size > 0 => {
                                    let msg = String::from_utf8_lossy(&buf[..size]).to_string();
                                    let _ = tx.send(ThreadMessage::MessageReceived(msg));
                                }
                                Ok(_) => break,
                                Err(e) => {
                                    let _ = tx.send(ThreadMessage::Error(e.to_string()));
                                    break;
                                }
                            }
                        }
                    });
                }
                ThreadMessage::Connected(Err(e)) => {
                    self.error_message = Some(format!("Connection error: {}", e));
                }
                ThreadMessage::MessageReceived(msg) => {
                    self.messages.push(msg);
                }
                ThreadMessage::Error(e) => {
                    self.messages.push(format!("Error: {}", e));
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| match self.state {
            AppState::Connect => self.show_connect_screen(ui),
            AppState::Chat => self.show_chat_screen(ui, ctx),
        });
    }
}

impl ChatApp {
    fn show_connect_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Connect to Server");
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label("IP:");
                ui.text_edit_singleline(&mut self.ip);
            });
            ui.horizontal(|ui| {
                ui.label("Port:");
                ui.text_edit_singleline(&mut self.port);
            });
            ui.add_space(10.0);
            if ui.button("Connect").clicked() {
                self.try_connect();
            }
            if let Some(err) = &self.error_message {
                ui.colored_label(egui::Color32::RED, err);
            }
        });
    }

    fn show_chat_screen(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical(|ui| {
            ui.heading("Chat");
            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(ui.available_height() - 40.0)
                .show(ui, |ui| {
                    for msg in &self.messages {
                        ui.label(msg);
                    }
                });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.separator();
                ui.horizontal(|ui| {
                    let text_edit = ui.add(
                        egui::TextEdit::singleline(&mut self.message_input)
                            .desired_width(ui.available_width() - 60.0),
                    );

                    if text_edit.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.send_message();
                        text_edit.request_focus();
                    }

                    if ui.button("Send").clicked() {
                        self.send_message();
                    }
                });
            });
        });
    }

    fn send_message(&mut self) {
        if let Some(stream) = &mut self.stream {
            let msg = self.message_input.clone();
            if let Err(e) = stream.write_all(msg.as_bytes()) {
                self.messages.push(format!("Send error: {}", e));
            } else {
                let _ = stream.flush();
                self.messages.push(format!("You: {}", msg));
                self.message_input.clear();
            }
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Chat Application",
        options,
        Box::new(|_cc| Ok(Box::new(ChatApp::new()))),
    );
}
