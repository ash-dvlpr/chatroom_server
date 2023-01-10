mod config;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

// ============== Logic ===============
pub async fn open_socket() -> TcpListener {
    TcpListener::bind(config::SOCKET_ADDR).await.unwrap()
}

pub async fn handle_connection(mut tcp_stream: TcpStream) {
    // Buffered Reader to handle the connection's stream
    let (reader, mut writer) = tcp_stream.split();
    let mut reader = BufReader::new(reader);
    
    // Buffer for messages
    let mut line = String::new();

    // Handle incoming messages
    loop {
        // Prompt
        writer.write(r"> ".as_bytes()).await.unwrap();
        
        // Read message
        let bytes_read = 
            reader.read_line(&mut line).await.unwrap_or(0);
        if bytes_read == 0 { break; } // Client Disconnected

        writer.write_all(line.as_bytes()).await.unwrap();
        line.clear(); // Clear buffer
    }
}
