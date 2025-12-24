use crate::services::websocket::send_decompile_script;
use crate::state::{ActiveExplorerClient, ClientRegistry};
use crate::utils::client_helpers::get_active_client;
use tauri::State;

#[tauri::command]
pub async fn exp_decompile(
    id: u32,
    active_explorer: State<'_, ActiveExplorerClient>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_client(&active_explorer, "explorer").await?;
    send_decompile_script(&client_id, id, &clients).await
}
