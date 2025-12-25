use crate::models::remote_spy::*;
use crate::state::ClientRegistry;
use crate::utils::events::emit_or_log;
use serde::Serialize;
use tauri::AppHandle;

use super::super::client_manager::send_to_client;
use super::super::messages::ServerMessage;

/// Emit a remote spy event to the frontend
pub fn emit_remote_spy_event<T: Serialize + Clone>(
    app_handle: &AppHandle,
    event_name: &str,
    payload: T,
) {
    emit_or_log(app_handle, event_name, payload);
}

/// Handle RspyCall message from client
pub fn handle_rspy_call(
    app_handle: &AppHandle,
    call_id: u32,
    remote_id: u32,
    name: String,
    path: String,
    class: String,
    direction: String,
    timestamp: String,
    arguments: Vec<RemoteArgument>,
    return_value: Option<RemoteArgument>,
    calling_script_name: Option<String>,
    calling_script_path: Option<String>,
) {
    emit_remote_spy_event(
        app_handle,
        "remote-spy-call",
        RemoteCallEvent {
            call_id,
            remote_id,
            name,
            path,
            class,
            direction,
            timestamp,
            arguments,
            return_value,
            calling_script_name,
            calling_script_path,
        },
    );
}

/// Handle RspyDecompiled message from client
pub fn handle_rspy_decompiled(app_handle: &AppHandle, script_path: String, source: String) {
    emit_remote_spy_event(
        app_handle,
        "remote-spy-decompiled",
        serde_json::json!({
            "scriptPath": script_path,
            "source": source,
        }),
    );
}

/// Handle RspyGeneratedCode message from client
pub fn handle_rspy_generated_code(app_handle: &AppHandle, call_id: u32, code: String) {
    emit_remote_spy_event(
        app_handle,
        "remote-spy-generated-code",
        serde_json::json!({
            "callId": call_id,
            "code": code,
        }),
    );
}

// Public API functions for sending messages to clients

/// Send rspy_start message to a client
pub async fn send_start_remote_spy(
    client_id: &str,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let msg = ServerMessage::RspyStart;
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize rspy_start message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}

/// Send rspy_stop message to a client
pub async fn send_stop_remote_spy(client_id: &str, clients: &ClientRegistry) -> Result<(), String> {
    let msg = ServerMessage::RspyStop;
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize rspy_stop message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}

/// Send rspy_decompile message to a client
pub async fn send_decompile_request(
    client_id: &str,
    script_path: String,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let msg = ServerMessage::RspyDecompile { script_path };
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize rspy_decompile message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}

/// Send rspy_generate_code message to a client
pub async fn send_generate_code_request(
    client_id: &str,
    call_id: u32,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let msg = ServerMessage::RspyGenerateCode { call_id };
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize rspy_generate_code message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}
