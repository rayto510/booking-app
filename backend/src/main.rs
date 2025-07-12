use backend::app;
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::serve(TcpListener::bind(addr).await.unwrap(), app()).await.unwrap();
}
