mod config;

use tokio::net::TcpListener;

// ============== Functions ===============
pub async fn open_socket() -> TcpListener {
    TcpListener::bind(config::SOCKET_ADDR).await.unwrap()
}