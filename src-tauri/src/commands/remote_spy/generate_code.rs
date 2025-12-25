use crate::services::websocket::send_generate_code_request;
use crate::state::{ActiveRemoteSpyClient, ClientRegistry};
use crate::utils::client_helpers::get_active_client;
use tauri::State;

#[tauri::command]
pub async fn rspy_generate_code(
    call_id: u32,
    active_remote_spy: State<'_, ActiveRemoteSpyClient>,
    clients: State<'_, ClientRegistry>,
) -> Result<(), String> {
    let client_id = get_active_client(&active_remote_spy, "remote spy").await?;
    send_generate_code_request(&client_id, call_id, &clients).await
}
