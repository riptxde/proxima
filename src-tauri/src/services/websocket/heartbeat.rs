use crate::state::ClientRegistry;
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;

use super::client_manager::emit_clients_update;
use super::messages::ServerMessage;

// Heartbeat configuration
const PING_INTERVAL: f64 = 5.0; // seconds
const MAX_MISSED_CYCLES: u32 = 2;

/// Start the heartbeat monitoring task for a client
pub fn start_heartbeat_monitor(
    tx: UnboundedSender<Message>,
    missed_pings: Arc<RwLock<u32>>,
    client_id_shared: Arc<RwLock<Option<String>>>,
    clients: ClientRegistry,
    app_handle: AppHandle,
) -> tauri::async_runtime::JoinHandle<()> {
    tauri::async_runtime::spawn(async move {
        let mut interval = interval(Duration::from_secs_f64(PING_INTERVAL));
        interval.tick().await; // Skip first immediate tick

        loop {
            interval.tick().await;

            let client_id_opt = client_id_shared.read().await.clone();
            let current_missed = *missed_pings.read().await;

            // Check missed pings
            if current_missed >= MAX_MISSED_CYCLES {
                if let Some(id) = &client_id_opt {
                    log::warn!(
                        "Client {} failed to respond after {} cycles, disconnecting",
                        id,
                        MAX_MISSED_CYCLES
                    );
                }
                break;
            }

            // Increment missed pings counter
            *missed_pings.write().await += 1;

            // Send ping
            let ping_msg = ServerMessage::Ping;
            let ping_text = match serde_json::to_string(&ping_msg) {
                Ok(text) => text,
                Err(_) => break,
            };

            if let Some(id) = &client_id_opt {
                log::debug!("Sending ping to {} (missed: {})", id, current_missed);
            }

            if tx.send(Message::Text(ping_text)).is_err() {
                if let Some(id) = &client_id_opt {
                    log::warn!("Failed to send ping to {}, connection likely closed", id);
                }
                break;
            }
        }

        // Cleanup on heartbeat failure - forcefully close connection
        if let Some(id) = client_id_shared.read().await.as_ref() {
            clients.write().await.remove(id);
            log::info!("Removed dead client: {}", id);

            // Send close message to forcefully disconnect
            let _ = tx.send(Message::Close(None));

            emit_clients_update(&app_handle, &clients).await;
        }
    })
}
