//! Relay message handler
//!
//! Handles relaying messages from one client to all other clients

use crate::state::ClientRegistry;
use tokio_tungstenite::tungstenite::Message;

use super::super::messages::ServerMessage;

/// Handle relay message from a client and relay to all other clients
pub async fn handle_relay(sender_id: &str, content: String, clients: &ClientRegistry) {
    let message = ServerMessage::Relay { content };

    let json = match serde_json::to_string(&message) {
        Ok(json) => json,
        Err(e) => {
            log::error!("Failed to serialize relay message: {}", e);
            return;
        }
    };

    let clients_read = clients.read().await;

    // Send to all clients except the sender
    for (client_id, client_info) in clients_read.iter() {
        if client_id != sender_id {
            if let Err(e) = client_info.sender.send(Message::Text(json.clone())) {
                log::error!(
                    "Failed to send relay to client {}: {}",
                    client_info.username,
                    e
                );
            }
        }
    }
}
