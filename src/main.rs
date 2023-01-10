use chatroom_server::*; // lib.rs

#[tokio::main]
async fn main() {
    let listener = open_socket().await;

    // Handle incoming connections
    loop {
        let (socket, _addr) = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(socket));
    }
}
