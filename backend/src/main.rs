use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

mod config;
mod error;
mod routes;
mod signaling;
mod state;

use state::{AppState, SharedState};

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(AppState::default());

    let app = Router::new()
        .route("/health", axum::routing::get(|| async { "Ok!" }))
        .merge(routes::app(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("сервер запущен на http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
