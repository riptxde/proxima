use crate::services::websocket::send_decompile_script;
use crate::state::{ActiveClientsState, ClientRegistry};
use crate::utils::clients::get_active_explorer;
use tauri::State;

#[tauri::command]
pub async fn exp_decompile(
    id: u32,
    active_clients: State<'_, ActiveClientsState>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_explorer(&active_clients).await?;
    send_decompile_script(&client_id, id, &clients).await
}
