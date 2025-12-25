use crate::services::websocket::send_decompile_request;
use crate::state::{ActiveClientsState, ClientRegistry};
use crate::utils::clients::get_active_remote_spy;
use tauri::State;

#[tauri::command]
pub async fn rspy_decompile(
    script_path: String,
    active_clients: State<'_, ActiveClientsState>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_remote_spy(&active_clients).await?;
    send_decompile_request(&client_id, script_path, &clients).await
}
