use crate::services::websocket::send_get_explorer_properties;
use crate::state::{ActiveClientsState, ApiDumpCache, ClientRegistry};
use crate::utils::clients::get_active_explorer;
use tauri::State;

#[tauri::command]
pub async fn exp_get_properties(
    id: u32,
    class_name: String,
    active_clients: State<'_, ActiveClientsState>,
    clients: State<'_, ClientRegistry>,
    api_dump: State<'_, ApiDumpCache>,
) -> Result<(), String> {
    let client_id = get_active_explorer(&active_clients).await?;

    // Get properties from API dump
    let (properties, special_properties) = {
        let service = api_dump.read().await;
        if service.is_loaded() {
            service.get_class_properties(&class_name)
        } else {
            log::warn!("API dump not loaded yet, sending empty property lists");
            (vec![], vec![])
        }
    };

    send_get_explorer_properties(&client_id, id, properties, special_properties, &clients).await
}
