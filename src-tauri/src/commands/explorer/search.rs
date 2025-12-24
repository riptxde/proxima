use crate::services::websocket::send_search_explorer;
use crate::state::{ActiveExplorerClient, ClientRegistry};
use crate::utils::client_helpers::get_active_client;
use tauri::State;

#[tauri::command]
pub async fn exp_search(
    query: String,
    search_by: String,
    limit: u32,
    active_explorer: State<'_, ActiveExplorerClient>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_client(&active_explorer, "explorer").await?;
    send_search_explorer(&client_id, query, search_by, limit, &clients).await
}
