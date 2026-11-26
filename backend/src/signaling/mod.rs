pub mod manager;
pub mod messages;
//pub mod peer;
//pub mod room;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};

use crate::state::SharedState;

pub fn router(state: SharedState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = socket.split();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let mut current_room_id: Option<String> = None;
    let mut current_user_id: Option<String> = None;

    let state_clone = state.clone();
    let tx_clone = tx.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(msg) =
                serde_json::from_str::<crate::signaling::messages::ClientMessage>(&text)
            {
                crate::signaling::manager::handle_client_message(
                    msg,
                    &state_clone,
                    &tx_clone,
                    &mut current_room_id,
                    &mut current_user_id,
                )
                .await;
            }
        }

        if let (Some(room), Some(user)) = (current_room_id, current_user_id) {
            crate::signaling::manager::remove_user(&state_clone, &room, &user).await;
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
