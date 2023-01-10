use chatroom_server::*; // lib.rs
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};

#[tokio::main]
async fn main() {
    let listener = open_socket().await;

    // Establish a connection
    let (socket, _addr) = listener.accept().await.unwrap();

    // Handle connection
    let mut socket = BufReader::new(socket);
    let mut buffer = String::new();

    // ECHO message
    socket.read_line(&mut buffer).await.unwrap();
    socket.write_all(buffer.as_bytes()).await.unwrap();
}
