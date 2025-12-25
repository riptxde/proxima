use crate::services::websocket::send_get_explorer_tree;
use crate::state::{ActiveClientsState, ClientRegistry};
use crate::utils::clients::get_active_explorer;
use tauri::State;

#[tauri::command]
pub async fn exp_get_tree(
    expanded_ids: Vec<u32>,
    active_clients: State<'_, ActiveClientsState>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_explorer(&active_clients).await?;
    send_get_explorer_tree(&client_id, expanded_ids, &clients).await
}
