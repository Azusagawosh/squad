pub mod health;
pub mod rooms;
pub mod ws;

use axum::Router;
use crate::state::SharedState;

pub fn app(state: SharedState) -> Router {
    Router::new().merge(crate::signaling::router(state.clone()))
}
