//! Rust backend scaffold using Axum for REST + WebSocket sync

use std::{ net::SocketAddr };

pub mod handlers;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    let app = routes::app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Rayflow backend running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
