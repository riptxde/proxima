use crate::models::LogMessage;
use crate::utils::events::emit_or_log;
use tauri::AppHandle;

/// Handle log message from client
pub fn handle_log(app_handle: &AppHandle, level: u8, message: String) {
    let log_msg = LogMessage { level, message };
    emit_or_log(app_handle, "log-message", &log_msg);
}
