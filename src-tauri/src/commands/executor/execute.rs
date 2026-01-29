use crate::models::ExecuteRequest;
use crate::state::ClientRegistry;
use tauri::State;

#[tauri::command]
pub async fn exec(
    request: ExecuteRequest,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    // Validate script is not empty
    if request.script.trim().is_empty() {
        return Err("Script cannot be empty".to_string());
    }

    // Validate at least one client is selected
    if request.client_ids.is_empty() {
        return Err("No clients selected for execution".to_string());
    }

    // Broadcast to selected clients
    crate::services::websocket::broadcast_to_clients(
        request.client_ids,
        request.script,
        request.redirect,
        clients.inner().clone(),
    )
    .await?;

    Ok(())
}
