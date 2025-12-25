use crate::services::websocket::send_search_explorer;
use crate::state::{ActiveClientsState, ClientRegistry};
use crate::utils::clients::get_active_explorer;
use tauri::State;

#[tauri::command]
pub async fn exp_search(
    query: String,
    search_by: String,
    limit: u32,
    active_clients: State<'_, ActiveClientsState>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_explorer(&active_clients).await?;
    send_search_explorer(&client_id, query, search_by, limit, &clients).await
}
