use crate::models::Client;
use crate::state::ClientRegistry;
use tauri::State;

#[tauri::command]
pub async fn get_attached_clients(
    clients: State<'_, ClientRegistry>,
) -> Result<Vec<Client>, String> {
    Ok(crate::services::websocket::get_attached_clients(clients.inner().clone()).await)
}
