use crate::models::ExecuteRequest;
use crate::services::websocket::ClientRegistry;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub async fn execute_script(
    app: AppHandle,
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

    let client_count = request.client_ids.len();

    // Broadcast to selected clients
    crate::services::websocket::broadcast_to_clients(
        request.client_ids,
        request.script,
        clients.inner().clone(),
    )
    .await?;

    // Log successful execution
    let client_text = if client_count == 1 {
        "1 client".to_string()
    } else {
        format!("{} clients", client_count)
    };

    let _ = app.emit("log-message", serde_json::json!({
        "level": 1,
        "message": format!("Script ran on {}", client_text)
    }));

    Ok(())
}
