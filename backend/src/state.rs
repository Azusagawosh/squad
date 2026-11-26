use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

pub type UserId = String;
pub type RoomId = String;
pub type Tx = mpsc::UnboundedSender<String>;

#[derive(Default)]
pub struct AppState {
    pub rooms: RwLock<HashMap<RoomId, HashMap<UserId, Tx>>>,
}

pub type SharedState = Arc<AppState>;
