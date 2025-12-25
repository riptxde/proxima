use crate::services::websocket::{send_start_remote_spy, send_stop_remote_spy};
use crate::state::{ActiveClientsState, ClientRegistry};
use crate::utils::clients::verify_client_exists;
use crate::utils::events::emit_or_error;
use tauri::State;

#[tauri::command]
pub async fn rspy_start(
    client_id: String,
    clients: State<'_, ClientRegistry>,
    active_clients: State<'_, ActiveClientsState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    log::info!("Starting remote spy for client: {}", client_id);

    // Check if another client already has remote spy active
    {
        let active = active_clients.read().await;
        if let Some(existing_id) = &active.remote_spy {
            if existing_id != &client_id {
                return Err(format!(
                    "Remote spy is already active for another client: {}",
                    existing_id
                ));
            }
            // Already active for this client
            return Ok(());
        }
    }

    // Verify client exists
    verify_client_exists(&client_id, &clients).await?;

    // Set as active remote spy client
    {
        let mut active = active_clients.write().await;
        active.remote_spy = Some(client_id.clone());
    }

    // Send start_remote_spy message to client
    send_start_remote_spy(&client_id, &clients).await?;

    log_ui!(&app, Info, "Starting remote spy for client: {}", client_id);

    // Emit event to frontend
    emit_or_error(&app, "remote-spy-started", client_id.clone())?;

    Ok(())
}

#[tauri::command]
pub async fn rspy_stop(
    active_clients: State<'_, ActiveClientsState>,
    clients: State<'_, ClientRegistry>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    log::info!("Stopping remote spy");

    let client_id = {
        let mut active = active_clients.write().await;
        active.remote_spy.take()
    };

    if let Some(id) = client_id {
        // Send stop_remote_spy message to client (ignore errors if client disconnected)
        let _ = send_stop_remote_spy(&id, &clients).await;

        // Emit event to frontend
        emit_or_error(&app, "remote-spy-stopped", ())?;
    }

    Ok(())
}
