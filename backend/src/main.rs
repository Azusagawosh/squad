use axum::{routing::get, Router};

mod config;
mod error;
mod routes;
mod signaling;
mod state;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "Ok!" }))
        .route("/ping", get(|| async { "pong" }));
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config::PORT));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("сервер запушен на http://{}", addr);
    axum::serve(listener, app,).await.unwrap();
}
