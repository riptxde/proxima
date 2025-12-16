use crate::services::websocket::{
    send_decompile_script, send_get_explorer_properties, send_get_explorer_tree,
    send_search_explorer, send_start_explorer, send_stop_explorer, ActiveExplorerClient,
    ApiDumpCache, ClientRegistry,
};
use tauri::{Emitter, State};

/// Helper to get the active explorer client ID
async fn get_active_client_id(active_explorer: &ActiveExplorerClient) -> Result<String, String> {
    active_explorer
        .read()
        .await
        .as_ref()
        .cloned()
        .ok_or_else(|| "No active explorer client".to_string())
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
pub async fn start_explorer(
    client_id: String,
    clients: State<'_, ClientRegistry>,
    active_explorer: State<'_, ActiveExplorerClient>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    log::info!("Starting explorer for client: {}", client_id);

    // Check if another client already has explorer active
    {
        let active = active_explorer.read().await;
        if let Some(existing_id) = active.as_ref() {
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
        let mut active = active_explorer.write().await;
        *active = Some(client_id.clone());
    }

    // Send start_explorer message to client
    send_start_explorer(&client_id, &clients).await?;

    log_ui!(&app, Info, "Starting explorer for client: {}", client_id);

    // Emit event to frontend
    app.emit("explorer-started", client_id.clone())
        .map_err(|e| format!("Failed to emit explorer-started event: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn stop_explorer(
    active_explorer: State<'_, ActiveExplorerClient>,
    clients: State<'_, ClientRegistry>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    log::info!("Stopping explorer");

    let client_id = {
        let mut active = active_explorer.write().await;
        active.take()
    };

    if let Some(id) = client_id {
        // Send stop_explorer message to client (ignore errors if client disconnected)
        let _ = send_stop_explorer(&id, &clients).await;

        // Emit event to frontend
        app.emit("explorer-stopped", ())
            .map_err(|e| format!("Failed to emit explorer-stopped event: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn explorer_get_tree(
    expanded_ids: Vec<u32>,
    active_explorer: State<'_, ActiveExplorerClient>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_client_id(&active_explorer).await?;
    send_get_explorer_tree(&client_id, expanded_ids, &clients).await
}

#[tauri::command]
pub async fn explorer_get_properties(
    id: u32,
    class_name: String,
    active_explorer: State<'_, ActiveExplorerClient>,
    clients: State<'_, ClientRegistry>,
    api_dump: State<'_, ApiDumpCache>,
) -> Result<(), String> {
    let client_id = get_active_client_id(&active_explorer).await?;

    // Get properties from API dump
    let (properties, special_properties) = {
        let service = api_dump.read().await;
        if service.is_loaded() {
            service.get_class_properties(&class_name)
        } else {
            log::warn!("API dump not loaded yet, sending empty property lists");
            (vec![], vec![])
        }
    };

    send_get_explorer_properties(&client_id, id, properties, special_properties, &clients).await
}

#[tauri::command]
pub async fn explorer_search(
    query: String,
    search_by: String,
    limit: u32,
    active_explorer: State<'_, ActiveExplorerClient>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_client_id(&active_explorer).await?;
    send_search_explorer(&client_id, query, search_by, limit, &clients).await
}

#[tauri::command]
pub async fn explorer_decompile_script(
    id: u32,
    active_explorer: State<'_, ActiveExplorerClient>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_client_id(&active_explorer).await?;
    send_decompile_script(&client_id, id, &clients).await
}
