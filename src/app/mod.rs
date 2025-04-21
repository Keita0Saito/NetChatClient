mod connection;
mod message;

use crate::{net, ui};
pub use connection::AppState;
use eframe::egui;
pub use message::ThreadMessage;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct ChatApp {
    pub state: AppState,
    pub ip: String,
    pub port: String,
    pub error_message: Option<String>,
    pub message_input: String,
    pub messages: Vec<String>,
    pub stream: Option<TcpStream>,
    pub tx: Sender<ThreadMessage>,
    pub rx: Receiver<ThreadMessage>,
}

impl ChatApp {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            state: AppState::Connect,
            ip: "127.0.0.1".into(),
            port: "8080".into(),
            error_message: None,
            message_input: String::new(),
            messages: Vec::new(),
            stream: None,
            tx,
            rx,
        }
    }

    pub fn try_connect(&mut self) {
        net::try_connect(self.ip.clone(), self.port.clone(), self.tx.clone());
    }

    pub fn disconnect(&mut self) {
        self.stream = None;
        self.state = AppState::Connect;
        self.messages.clear();
    }

    pub fn send_message(&mut self) {
        if let Some(stream) = &mut self.stream {
            net::send_message(stream, &self.message_input, &mut self.messages);
            self.message_input.clear();
        }
    }
}

impl eframe::App for ChatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        net::handle_messages(self);
        ui::show_main_panel(self, ctx);
    }
}
