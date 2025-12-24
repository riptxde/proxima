use crate::models::remote_spy::RemoteArgument;
use crate::services::websocket::{
    send_decompile_request, send_generate_code_request, send_start_remote_spy,
    send_stop_remote_spy, ActiveRemoteSpyClient, ClientRegistry,
};
use tauri::{Emitter, State};

/// Helper to get the active remote spy client ID
async fn get_active_client_id(active_remote_spy: &ActiveRemoteSpyClient) -> Result<String, String> {
    active_remote_spy
        .read()
        .await
        .as_ref()
        .cloned()
        .ok_or_else(|| "No active remote spy client".to_string())
}

/// Helper to verify client exists in registry
async fn verify_client_exists(client_id: &str, clients: &ClientRegistry) -> Result<(), String> {
    let clients_lock = clients.read().await;
    if clients_lock.contains_key(client_id) {
        Ok(())
    } else {
        Err(format!("Client not found: {}", client_id))
    }
}

#[tauri::command]
pub async fn rspy_start(
    client_id: String,
    clients: State<'_, ClientRegistry>,
    active_remote_spy: State<'_, ActiveRemoteSpyClient>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    log::info!("Starting remote spy for client: {}", client_id);

    // Check if another client already has remote spy active
    {
        let active = active_remote_spy.read().await;
        if let Some(existing_id) = active.as_ref() {
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
        let mut active = active_remote_spy.write().await;
        *active = Some(client_id.clone());
    }

    // Send start_remote_spy message to client
    send_start_remote_spy(&client_id, &clients).await?;

    log_ui!(&app, Info, "Starting remote spy for client: {}", client_id);

    // Emit event to frontend
    app.emit("remote-spy-started", client_id.clone())
        .map_err(|e| format!("Failed to emit remote-spy-started event: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn rspy_stop(
    active_remote_spy: State<'_, ActiveRemoteSpyClient>,
    clients: State<'_, ClientRegistry>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    log::info!("Stopping remote spy");

    let client_id = {
        let mut active = active_remote_spy.write().await;
        active.take()
    };

    if let Some(id) = client_id {
        // Send stop_remote_spy message to client (ignore errors if client disconnected)
        let _ = send_stop_remote_spy(&id, &clients).await;

        // Emit event to frontend
        app.emit("remote-spy-stopped", ())
            .map_err(|e| format!("Failed to emit remote-spy-stopped event: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn rspy_decompile(
    script_path: String,
    active_remote_spy: State<'_, ActiveRemoteSpyClient>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_client_id(&active_remote_spy).await?;
    send_decompile_request(&client_id, script_path, &clients).await
}

#[tauri::command]
pub async fn rspy_generate_code(
    call_id: String,
    name: String,
    path: String,
    remote_type: String,
    direction: String,
    arguments: Vec<RemoteArgument>,
    active_remote_spy: State<'_, ActiveRemoteSpyClient>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_client_id(&active_remote_spy).await?;
    send_generate_code_request(
        &client_id,
        call_id,
        name,
        path,
        remote_type,
        direction,
        arguments,
        &clients,
    )
    .await
}
