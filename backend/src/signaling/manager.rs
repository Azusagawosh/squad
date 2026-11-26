use crate::signaling::messages::{ClientMessage, ServerMessage};
use crate::state::{SharedState, Tx};
use serde_json::to_string;

pub async fn handle_client_message(
    msg: ClientMessage,
    state: &SharedState,
    tx: &Tx,
    current_room_id: &mut Option<String>,
    current_user_id: &mut Option<String>,
) {
    match msg {
        ClientMessage::Join { room_id, user_id } => {
            let mut rooms = state.rooms.write().await;
            let room = rooms
                .entry(room_id.clone())
                .or_insert_with(std::collections::HashMap::new);
            room.insert(user_id.clone(), tx.clone());

            *current_room_id = Some(room_id);
            *current_user_id = Some(user_id);
        }
        ClientMessage::Chat { text } => {
            if let (Some(room_id), Some(user_id)) = (&*current_room_id, &*current_user_id) {
                let rooms = state.rooms.read().await;
                if let Some(room) = rooms.get(room_id) {
                    let server_msg = ServerMessage::Chat {
                        sender_id: user_id.clone(),
                        text,
                    };
                    let json = to_string(&server_msg).unwrap();
                    for (_uid, user_tx) in room {
                        let _ = user_tx.send(json.clone());
                    }
                }
            }
        }
        ClientMessage::Offer { target_id, sdp } => {
            if let (Some(room_id), Some(user_id)) = (&*current_room_id, &*current_user_id) {
                let rooms = state.rooms.read().await;
                if let Some(room) = rooms.get(room_id) {
                    if let Some(target_tx) = room.get(&target_id) {
                        let server_msg = ServerMessage::Offer {
                            sender_id: user_id.clone(),
                            sdp,
                        };
                        let _ = target_tx.send(to_string(&server_msg).unwrap());
                    }
                }
            }
        }
        ClientMessage::Answer { target_id, sdp } => {
            if let (Some(room_id), Some(user_id)) = (&*current_room_id, &*current_user_id) {
                let rooms = state.rooms.read().await;
                if let Some(room) = rooms.get(room_id) {
                    if let Some(target_tx) = room.get(&target_id) {
                        let server_msg = ServerMessage::Answer {
                            sender_id: user_id.clone(),
                            sdp,
                        };
                        let _ = target_tx.send(to_string(&server_msg).unwrap());
                    }
                }
            }
        }
        ClientMessage::IceCandidate {
            target_id,
            candidate,
        } => {
            if let (Some(room_id), Some(user_id)) = (&*current_room_id, &*current_user_id) {
                let rooms = state.rooms.read().await;
                if let Some(room) = rooms.get(room_id) {
                    if let Some(target_tx) = room.get(&target_id) {
                        let server_msg = ServerMessage::IceCandidate {
                            sender_id: user_id.clone(),
                            candidate,
                        };
                        let _ = target_tx.send(to_string(&server_msg).unwrap());
                    }
                }
            }
        }
    }
}

pub async fn remove_user(state: &SharedState, room_id: &str, user_id: &str) {
    let mut rooms = state.rooms.write().await;
    if let Some(room) = rooms.get_mut(room_id) {
        room.remove(user_id);
        if room.is_empty() {
            rooms.remove(room_id);
        }
    }
}
