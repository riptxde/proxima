use crate::services::websocket::{send_start_explorer, send_stop_explorer};
use crate::state::{ActiveClientsState, ClientRegistry};
use crate::utils::clients::verify_client_exists;
use crate::utils::events::emit_or_error;
use tauri::State;

#[tauri::command]
pub async fn exp_start(
    client_id: String,
    clients: State<'_, ClientRegistry>,
    active_clients: State<'_, ActiveClientsState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    log::info!("Starting explorer for client: {}", client_id);

    // Check if another client already has explorer active
    {
        let active = active_clients.read().await;
        if let Some(existing_id) = &active.explorer {
            if existing_id != &client_id {
                return Err(format!(
                    "Explorer is already active for another client: {}",
                    existing_id
                ));
            }
            // Already active for this client
            return Ok(());
        }
    }

    // Verify client exists
    verify_client_exists(&client_id, &clients).await?;

    // Set as active explorer client
    {
        let mut active = active_clients.write().await;
        active.explorer = Some(client_id.clone());
    }

    // Send start_explorer message to client
    send_start_explorer(&client_id, &clients).await?;

    log_ui!(&app, Info, "Starting explorer for client: {}", client_id);

    // Emit event to frontend
    emit_or_error(&app, "explorer-started", client_id.clone())?;

    Ok(())
}

#[tauri::command]
pub async fn exp_stop(
    active_clients: State<'_, ActiveClientsState>,
    clients: State<'_, ClientRegistry>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    log::info!("Stopping explorer");

    let client_id = {
        let mut active = active_clients.write().await;
        active.explorer.take()
    };

    if let Some(id) = client_id {
        // Send stop_explorer message to client (ignore errors if client disconnected)
        let _ = send_stop_explorer(&id, &clients).await;

        // Emit event to frontend
        emit_or_error(&app, "explorer-stopped", ())?;
    }

    Ok(())
}
