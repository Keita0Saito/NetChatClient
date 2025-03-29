use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::mpsc::Sender,
    thread,
};
use crate::app::{AppState, ChatApp, ThreadMessage};

const READ_BUFFER_SIZE: usize = 1024;

pub fn try_connect(ip: String, port: String, tx: Sender<ThreadMessage>) {
    let addr = format!("{}:{}", ip, port);

    thread::spawn(move || {
        match TcpStream::connect(&addr) {
            Ok(stream) => {
                let stream_clone = stream.try_clone()
                    .expect("Failed to clone TCP stream");

                let _ = tx.send(ThreadMessage::Connected(Ok(stream_clone)));
                
                start_reading(stream, tx.clone());
            }

            Err(e) => {
                let _ = tx.send(ThreadMessage::Connected(Err(e.to_string())));
            }
        }
    });
}

fn start_reading(mut stream: TcpStream, tx: Sender<ThreadMessage>) {
    thread::spawn(move || {
        let mut buffer = [0u8, READ_BUFFER_SIZE.try_into().unwrap()];
        loop {
            match stream.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    let msg = String::from_utf8_lossy(&buffer[..size]).into();
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
) {
    if let Err(e) = stream.write_all(message.as_bytes()) {
        messages.push(format!("Send error: {}", e));
    } else {
        let _ = stream.flush();
        messages.push(format!("You: {}", message));
    }
}

pub fn handle_messages(app: &mut ChatApp) {
    while let Ok(msg) = app.rx.try_recv() {
        match msg {
            ThreadMessage::Connected(Ok(stream)) => {
                app.stream = Some(stream);
                app.state = AppState::Chat;
            }

            ThreadMessage::Connected(Err(e)) => {
                app.error_message = Some(format!("Connection error: {}", e)     );
            }

            ThreadMessage::MessageReceived(msg) => {
                app.messages.push(msg);
            }

            ThreadMessage::Error(e) => {
                app.messages.push(format!("Error: {}", e));
            }
        }
    }
}
