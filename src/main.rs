use chatroom_server::*; // lib.rs

#[tokio::main]
async fn main() {
    let listener = open_socket().await;

    // Establish a connection
    let (socket, _addr) = listener.accept().await.unwrap();

    handle_connection(socket).await;
}
