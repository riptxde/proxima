use crate::services::websocket::send_get_explorer_tree;
use crate::state::{ActiveExplorerClient, ClientRegistry};
use crate::utils::client_helpers::get_active_client;
use tauri::State;

#[tauri::command]
pub async fn exp_get_tree(
    expanded_ids: Vec<u32>,
    active_explorer: State<'_, ActiveExplorerClient>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_client(&active_explorer, "explorer").await?;
    send_get_explorer_tree(&client_id, expanded_ids, &clients).await
}
