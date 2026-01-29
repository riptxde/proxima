use crate::models::Client;
use crate::state::ClientRegistry;
use crate::utils::events::emit_or_log;
use tauri::AppHandle;
use tokio_tungstenite::tungstenite::Message;

use super::messages::ServerMessage;

/// Broadcast a script to multiple clients
pub async fn broadcast_to_clients(
    client_ids: Vec<String>,
    script: String,
    redirect: bool,
    clients: ClientRegistry,
) -> Result<(), String> {
    log::info!("Broadcasting script to {} client(s)", client_ids.len());

    let execute_msg = ServerMessage::Exec { script, redirect };
    let message_text = serde_json::to_string(&execute_msg)
        .map_err(|e| format!("Failed to serialize exec message: {}", e))?;

    let clients_lock = clients.read().await;
    let mut executed_count = 0;

    for client_id in &client_ids {
        if let Some(client_info) = clients_lock.get(client_id) {
            if client_info
                .sender
                .send(Message::Text(message_text.clone()))
                .is_ok()
            {
                executed_count += 1;
                log::debug!("Sent to client: {}", client_id);
            } else {
                log::error!("Failed to send to client: {}", client_id);
            }
        } else {
            log::error!("Client not found: {}", client_id);
        }
    }

    log::info!(
        "Successfully sent to {}/{} clients",
        executed_count,
        client_ids.len()
    );

    Ok(())
}

/// Get list of all attached clients
pub async fn get_attached_clients(clients: ClientRegistry) -> Vec<Client> {
    clients
        .read()
        .await
        .iter()
        .map(|(id, info)| Client {
            id: id.clone(),
            username: info.username.clone(),
        })
        .collect()
}

/// Send a message to a specific client
pub async fn send_to_client(
    client_id: &str,
    message: &str,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let clients_lock = clients.read().await;

    if let Some(client_info) = clients_lock.get(client_id) {
        client_info
            .sender
            .send(Message::Text(message.to_string()))
            .map_err(|e| format!("Failed to send message to client: {}", e))?;
        Ok(())
    } else {
        Err(format!("Client not found: {}", client_id))
    }
}

/// Emit clients-update event to frontend
pub async fn emit_clients_update(app_handle: &AppHandle, clients: &ClientRegistry) {
    let clients_list: Vec<Client> = clients
        .read()
        .await
        .iter()
        .map(|(id, info)| Client {
            id: id.clone(),
            username: info.username.clone(),
        })
        .collect();

    emit_or_log(app_handle, "clients-update", clients_list);
}
