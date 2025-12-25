use crate::services::websocket::send_generate_code_request;
use crate::state::{ActiveClientsState, ClientRegistry};
use crate::utils::clients::get_active_remote_spy;
use tauri::State;

#[tauri::command]
pub async fn rspy_generate_code(
    call_id: u32,
    active_clients: State<'_, ActiveClientsState>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_remote_spy(&active_clients).await?;
    send_generate_code_request(&client_id, call_id, &clients).await
}
