use serde::Serialize;
use tauri::{AppHandle, Emitter};

/// Emit an event and log errors instead of returning them
///
/// Use this when event emission failures should be logged but not interrupt the flow.
/// Common for non-critical UI updates.
///
/// # Arguments
/// * `app` - The Tauri app handle
/// * `event` - The event name
/// * `payload` - The event payload
pub fn emit_or_log<T: Serialize + Clone>(app: &AppHandle, event: &str, payload: T) {
    if let Err(e) = app.emit(event, payload) {
        log::error!("Failed to emit {} event: {}", event, e);
    }
}

/// Emit an event and return an error if it fails
///
/// Use this when event emission is critical and failures should be propagated.
///
/// # Arguments
/// * `app` - The Tauri app handle
/// * `event` - The event name
/// * `payload` - The event payload
///
/// # Returns
/// * `Ok(())` - Event emitted successfully
/// * `Err(String)` - Failed to emit event with error message
pub fn emit_or_error<T: Serialize + Clone>(
    app: &AppHandle,
    event: &str,
    payload: T,
) -> Result<(), String> {
    app.emit(event, payload)
        .map_err(|e| format!("Failed to emit {} event: {}", event, e))
}
