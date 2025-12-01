use crate::models::Client;
use crate::services::websocket::ClientRegistry;
use tauri::State;

#[tauri::command]
pub async fn get_connected_clients(
    clients: State<'_, ClientRegistry>,
) -> Result<Vec<Client>, String> {
    Ok(crate::services::websocket::get_connected_clients(clients.inner().clone()).await)
}
