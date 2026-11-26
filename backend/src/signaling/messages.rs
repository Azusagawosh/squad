use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ClientMessage {
    Join { room_id: String, user_id: String },
    Chat { text: String },
    Offer { target_id: String, sdp: String },
    Answer { target_id: String, sdp: String },
    IceCandidate { target_id: String, candidate: String },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    Error { message: String },
    Chat { sender_id: String, text: String },
    Offer { sender_id: String, sdp: String },
    Answer { sender_id: String, sdp: String },
    IceCandidate { sender_id: String, candidate: String },
}
