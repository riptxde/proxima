use crate::services::autoexec;
use crate::utils::paths;
use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

use super::super::messages::ServerMessage;

/// Handle the "ready" message from a client
pub async fn handle_ready(tx: &UnboundedSender<Message>, app_handle: &AppHandle) {
    log::info!("Client ready, sending auto-execute scripts");

    // Check if auto-execute is enabled
    let (auto_execute, redirect_print) = get_execution_settings(app_handle).await;

    if auto_execute {
        // Get autoexec scripts
        let scripts = autoexec::get_autoexec_scripts(app_handle);

        if !scripts.is_empty() {
            let script_count = scripts.len();

            // Log autoexec
            let script_text = if script_count == 1 {
                "1 script".to_string()
            } else {
                format!("{} scripts", script_count)
            };
            log_ui!(
                app_handle,
                Success,
                "Auto-executing {} on new client",
                script_text
            );

            // Execute each script on this client
            for script in scripts {
                let execute_msg = ServerMessage::Exec {
                    script,
                    redirect: redirect_print,
                };
                if let Ok(msg_text) = serde_json::to_string(&execute_msg) {
                    if tx.send(Message::Text(msg_text)).is_err() {
                        log::error!("Failed to send autoexec script");
                        break;
                    }
                }
            }
        }
    }
}

/// Read the execution settings from the Tauri store
/// Returns (autoExecute, redirectPrint)
async fn get_execution_settings(app: &AppHandle) -> (bool, bool) {
    // Get the base directory (same as scripts/autoexec location)
    let base_dir = match paths::get_base_directory(app) {
        Ok(dir) => dir,
        Err(e) => {
            log::error!("Failed to get base directory: {}", e);
            return (true, false); // Default to auto_execute=true, redirect=false on error
        }
    };

    let settings_path = base_dir.join("settings.json");
    let settings_path_str = settings_path.to_string_lossy().to_string();

    match app.store(&settings_path_str) {
        Ok(store) => match store.get("settings") {
            Some(Value::Object(settings)) => {
                if let Some(Value::Object(execution)) = settings.get("execution") {
                    let auto_execute = execution
                        .get("autoExecute")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true);
                    let redirect_print = execution
                        .get("redirectPrint")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    return (auto_execute, redirect_print);
                }
            }
            _ => {}
        },
        Err(e) => {
            log::error!("Failed to access settings store: {}", e);
        }
    }

    // Default to auto_execute=true, redirect=false if settings not found
    (true, false)
}
