use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Clone)]
struct LogMessage {
    level: u8,
    message: String,
}

#[tauri::command]
pub fn add_log(app: AppHandle, level: u8, message: String) -> Result<(), String> {
    // Validate level is 0-3 (info, success, warning, error)
    if level > 3 {
        return Err(format!("Invalid log level: {}. Must be 0-3", level));
    }

    // Emit log-message event to frontend
    let log_msg = LogMessage { level, message };

    app.emit("log-message", log_msg)
        .map_err(|e| format!("Failed to emit log event: {}", e))?;

    Ok(())
}
