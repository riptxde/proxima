use crate::state::ClientRegistry;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Helper to get the active client ID from any feature's active state
///
/// # Arguments
/// * `active_state` - The Arc<RwLock<Option<String>>> holding the active client ID
/// * `feature_name` - Name of the feature for error messages (e.g., "explorer", "remote spy")
///
/// # Returns
/// * `Ok(String)` - The active client ID
/// * `Err(String)` - If no client is active
pub async fn get_active_client(
    active_state: &Arc<RwLock<Option<String>>>,
    feature_name: &str,
) -> Result<String, String> {
    active_state
        .read()
        .await
        .as_ref()
        .cloned()
        .ok_or_else(|| format!("No active {} client", feature_name))
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
