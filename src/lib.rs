mod config;

type Connection = (TcpStream, SocketAddr);
type Message = (String, SocketAddr);

// =========== Dependencies ===========
use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast::{self, Receiver, Sender},
};

// ============== Logic ===============
pub async fn open_socket() -> TcpListener {
    TcpListener::bind(config::SERVER_ADDR).await.unwrap()
}
pub fn open_channel() -> (Sender<Message>, Receiver<Message>) {
    broadcast::channel::<Message>(config::CHANNEL_CAPACITY)
}

pub async fn handle_connection(connection: Connection, tx: Sender<Message>) {
    let (mut tcp_stream, client_addr) = connection;

    // Buffered Reader to handle the connection's stream
    let (reader, mut writer) = tcp_stream.split();
    let mut reader = BufReader::new(reader);

    // Buffer for messages
    let mut line = String::new();

    // Channel for comunication between Clients
    let mut rx = tx.subscribe();

    loop {
        // Handle incoming and outgoing messages
        tokio::select! {
            // Read Client message
            result = reader.read_line(&mut line) => {
                if result.unwrap_or(0) == 0 || line.starts_with(config::EXIT_COMMAND) {
                    break; // Close connection if the client disconnected or sent exit command
                }

                // Send message to other clients
                tx.send((line.clone(), client_addr)).unwrap();
                line.clear(); // Clear the buffer
            }
            // Pass other Client's messages
            result = rx.recv() => {
                let (message, addr) = result.unwrap();
                if addr != client_addr {
                    writer.write_all(message.as_bytes()).await.unwrap();
                }
            }
        }
    }
}
