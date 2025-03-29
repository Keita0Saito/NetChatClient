use std::net::TcpStream;

pub enum ThreadMessage {
    Connected(Result<TcpStream, String>),
    MessageReceived(String),
    Error(String),
}
