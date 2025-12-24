use crate::models::LogMessage;
use tauri::{AppHandle, Emitter};

/// Log levels that match the frontend
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum LogLevel {
    Info = 0,
    Success = 1,
    Warning = 2,
    Error = 3,
}

/// Emit a log message to the frontend UI
pub fn emit_log(app: &AppHandle, level: LogLevel, message: impl AsRef<str>) {
    let log_msg = LogMessage {
        level: level as u8,
        message: message.as_ref().to_string(),
    };

    if let Err(e) = app.emit("log-message", log_msg) {
        log::error!("Failed to emit log event: {}", e);
    }
}

/// Log to console with proper formatting and emit to UI
///
/// This macro combines console logging (with timestamps) and UI logging in one call.
///
/// # Examples
///
/// ```
/// // Info level - logs to console and UI
/// log_ui!(app, Info, "Server started on port {}", 13376);
///
/// // Success level
/// log_ui!(app, Success, "Client {} connected", username);
///
/// // Warning level
/// log_ui!(app, Warning, "Connection timeout for {}", client_id);
///
/// // Error level
/// log_ui!(app, Error, "Failed to start server: {}", error);
/// ```
#[macro_export]
macro_rules! log_ui {
    ($app:expr, Info, $($arg:tt)*) => {
        {
            let msg = format!($($arg)*);
            log::info!("{}", msg);
            $crate::utils::logging::emit_log($app, $crate::utils::logging::LogLevel::Info, &msg);
        }
    };
    ($app:expr, Success, $($arg:tt)*) => {
        {
            let msg = format!($($arg)*);
            log::info!("{}", msg);
            $crate::utils::logging::emit_log($app, $crate::utils::logging::LogLevel::Success, &msg);
        }
    };
    ($app:expr, Warning, $($arg:tt)*) => {
        {
            let msg = format!($($arg)*);
            log::warn!("{}", msg);
            $crate::utils::logging::emit_log($app, $crate::utils::logging::LogLevel::Warning, &msg);
        }
    };
    ($app:expr, Error, $($arg:tt)*) => {
        {
            let msg = format!($($arg)*);
            log::error!("{}", msg);
            $crate::utils::logging::emit_log($app, $crate::utils::logging::LogLevel::Error, &msg);
        }
    };
}
