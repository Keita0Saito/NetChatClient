use crate::app::{AppState, ChatApp, ThreadMessage};
use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::mpsc::Sender,
    thread,
};

const READ_BUFFER_SIZE: usize = 1024;

pub fn try_connect(ip: String, port: String, tx: Sender<ThreadMessage>) {
    let addr = format!("{}:{}", ip, port);

    thread::spawn(move || match TcpStream::connect(&addr) {
        Ok(stream) => {
            let stream_clone = stream.try_clone().expect("Failed to clone TCP stream");

            let _ = tx.send(ThreadMessage::Connected(Ok(stream_clone)));

            start_reading(stream, tx.clone());
        }
        Err(e) => {
            let _ = tx.send(ThreadMessage::Connected(Err(e.to_string())));
        }
    });
}

fn start_reading(mut stream: TcpStream, tx: Sender<ThreadMessage>) {
    thread::spawn(move || {
        let mut buffer = [0u8; READ_BUFFER_SIZE];
        loop {
            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let msg = String::from_utf8_lossy(&buffer[..size]).into_owned();
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

pub fn send_message(
    stream: &mut TcpStream,
    message: &str,
    messages: &mut Vec<String>,
) -> Result<(), String> {
    match stream.write_all(message.as_bytes()) {
        Ok(_) => {
            stream.flush().map_err(|e| e.to_string())?;
            messages.push(format!("You: {}", message));
            Ok(())
        }
        Err(e) => {
            messages.push(format!("Send error: {}", e));
            Err(e.to_string())
        }
    }
}

pub fn handle_messages(app: &mut ChatApp) {
    while let Ok(msg) = app.rx.try_recv() {
        match msg {
            ThreadMessage::Connected(Ok(stream)) => {
                app.stream = Some(stream);
                app.state = AppState::Chat;
                app.error_message = None;
            }
            ThreadMessage::Connected(Err(e)) => {
                app.error_message = Some(format!("Connection error: {}", e));
            }
            ThreadMessage::MessageReceived(msg) => {
                app.messages.push(msg);
            }
            ThreadMessage::Error(e) => {
                app.messages.push(format!("Error: {}", e));
                app.disconnect();
            }
        }
    }
}
