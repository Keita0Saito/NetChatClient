# Rust Chat Application  

A modern TCP chat client with GUI built in Rust using the `egui` framework.  

*Example: Connection and Chat Interface*

## Features ✨
- **GUI Interface** with egui/eframe
- **TCP Networking** with threaded I/O
- **Connection Management**  
  - IP/Port configuration
  - Error handling
  - Back button for connection reset
- **Real-Time Messaging**  
  - Message history with auto-scroll
  - Enter-to-send functionality
- **Modular Architecture**  
  - Isolated network layer
  - State-driven UI components

## Prerequisites 📦
- Rust 1.72+
- Cargo package manager
- TCP server (for testing: `nc -l 127.0.0.1 8080`)

## Installation ⚙️
```bash
git clone https://github.com/yourusername/rust-chat-app.git
cd rust-chat-app
cargo run --release 
```

## Usage 🖥️
### Connect to Server
- Enter server IP (default: 127.0.0.1)
- Enter port (default: 8080)
- Click Connect

### Chat Interface
- Type messages in bottom input
- Send with Enter or Send button
- History auto-updates

### Disconnect
- Click Back to reset connection
- Returns to connection screen

## Architecture Diagram 🗺️

```mermaid
%%{init: {'theme': 'dark'}}%%
flowchart TD
    subgraph UI Layer
        A[Connect Screen] -->|IP/Port| B[Chat Screen]
        B -->|Back Button| A
        B --> C[Settings Panel]
    end

    subgraph App Core
        B --> D[State Manager]
        D -->|Update| E[Network Handler]
        E -->|Threads| F[TCP Stream]
    end

    subgraph Server
        F --> G[Remote Server]
        G -->|Response| F
    end

    style A fill:#4CAF50,stroke:#333
    style B fill:#2196F3,stroke:#333
    style D fill:#9C27B0,stroke:#333
    style E fill:#FF9800,stroke:#333
```

**Key**:
- 🟢 **Green**: Connection interface  
- 🔵 **Blue**: Chat interface  
- 🟣 **Purple**: State controller  
- 🟠 **Orange**: Network module
