use crate::state::{ActiveClientsState, ClientRegistry};

/// Get the active explorer client ID
pub async fn get_active_explorer(active_clients: &ActiveClientsState) -> Result<String, String> {
    let active = active_clients.read().await;
    active
        .explorer
        .clone()
        .ok_or_else(|| "No active explorer client".to_string())
}

/// Get the active remote spy client ID
pub async fn get_active_remote_spy(active_clients: &ActiveClientsState) -> Result<String, String> {
    let active = active_clients.read().await;
    active
        .remote_spy
        .clone()
        .ok_or_else(|| "No active remote spy client".to_string())
}

/// Helper to verify a client exists in the registry
///
/// # Arguments
/// * `client_id` - The client ID to verify
/// * `clients` - The client registry
///
/// # Returns
/// * `Ok(())` - If the client exists
/// * `Err(String)` - If the client doesn't exist
pub async fn verify_client_exists(client_id: &str, clients: &ClientRegistry) -> Result<(), String> {
    let clients_lock = clients.read().await;
    if clients_lock.contains_key(client_id) {
        Ok(())
    } else {
        Err(format!("Client not found: {}", client_id))
    }
}
