use chatroom_server::*; // lib.rs

#[tokio::main]
async fn main() {
    let listener = open_socket().await;
    let (tx, _rx) = open_channel();

    // Handle incoming connections
    loop {
        let connection = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(connection, tx.clone()));
    }
}
