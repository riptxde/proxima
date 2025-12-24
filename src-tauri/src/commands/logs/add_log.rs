use crate::models::LogMessage;
use crate::utils::events::emit_or_error;
use tauri::AppHandle;

#[tauri::command]
pub fn add_log(app: AppHandle, level: u8, message: String) -> Result<(), String> {
    // Validate level is 0-3 (info, success, warning, error)
    if level > 3 {
        return Err(format!("Invalid log level: {}. Must be 0-3", level));
    }

    // Emit log-message event to frontend
    let log_msg = LogMessage { level, message };

    emit_or_error(&app, "log-message", log_msg)?;

    Ok(())
}
