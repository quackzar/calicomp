use std::net::SocketAddr;

use crate::ssh::AppServer;

pub mod http;
pub mod ssh;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tokio::spawn(async {
        let addr: SocketAddr = "127.0.0.1:1111".parse().unwrap();
        http::start(&addr).await.unwrap();
    });

    let mut server = AppServer::new();
    server.run().await.expect("Failed running server");
    tracing::info!("Started server");
}
