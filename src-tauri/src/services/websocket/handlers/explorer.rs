use crate::models::explorer::*;
use crate::state::ClientRegistry;
use crate::utils::events::emit_or_log;
use serde::Serialize;
use std::collections::HashMap;
use tauri::AppHandle;

use super::super::client_manager::send_to_client;
use super::super::messages::ServerMessage;

/// Emit an explorer event to the frontend
pub fn emit_explorer_event<T: Serialize + Clone>(
    app_handle: &AppHandle,
    event_name: &str,
    payload: T,
) {
    emit_or_log(app_handle, event_name, payload);
}

/// Handle ExpTree message from client
pub fn handle_exp_tree(app_handle: &AppHandle, nodes: Vec<ExplorerNode>) {
    emit_explorer_event(app_handle, "explorer-tree", TreeEvent { nodes });
}

/// Handle ExpProperties message from client
pub fn handle_exp_properties(
    app_handle: &AppHandle,
    id: u32,
    props: HashMap<String, PropertyData>,
    special_props: HashMap<String, PropertyData>,
) {
    emit_explorer_event(
        app_handle,
        "explorer-properties",
        PropertiesEvent {
            id,
            props,
            special_props,
        },
    );
}

/// Handle ExpSearchResults message from client
pub fn handle_exp_search_results(
    app_handle: &AppHandle,
    query: String,
    results: Vec<SearchResult>,
    total: u32,
    limited: bool,
) {
    emit_explorer_event(
        app_handle,
        "explorer-search-results",
        SearchResultsEvent {
            query,
            results,
            total,
            limited,
        },
    );
}

/// Handle ExpTreeChanged message from client
pub fn handle_exp_tree_changed(app_handle: &AppHandle) {
    emit_explorer_event(app_handle, "explorer-tree-changed", ());
}

/// Handle ExpDecompiled message from client
pub fn handle_exp_decompiled(app_handle: &AppHandle, id: u32, source: String) {
    emit_explorer_event(
        app_handle,
        "explorer-decompiled-script",
        DecompiledScriptEvent { id, source },
    );
}

// Public API functions for sending messages to clients

/// Send exp_start message to a client
pub async fn send_start_explorer(client_id: &str, clients: &ClientRegistry) -> Result<(), String> {
    let msg = ServerMessage::ExpStart;
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize exp_start message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}

/// Send exp_stop message to a client
pub async fn send_stop_explorer(client_id: &str, clients: &ClientRegistry) -> Result<(), String> {
    let msg = ServerMessage::ExpStop;
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize exp_stop message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}

/// Send exp_get_tree message to a client
pub async fn send_get_explorer_tree(
    client_id: &str,
    expanded_ids: Vec<u32>,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let msg = GetExplorerTreeMessage::new(expanded_ids);
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize exp_get_tree message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}

/// Send exp_get_properties message to a client
pub async fn send_get_explorer_properties(
    client_id: &str,
    id: u32,
    properties: Vec<crate::services::api_dump::PropertyMetadata>,
    special_properties: Vec<crate::services::api_dump::PropertyMetadata>,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let msg = GetExplorerPropertiesMessage::new(id, properties, special_properties);
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize exp_get_properties message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}

/// Send exp_search message to a client
pub async fn send_search_explorer(
    client_id: &str,
    query: String,
    search_by: String,
    limit: u32,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let msg = SearchExplorerMessage::new(query, search_by, limit);
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize exp_search message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}

/// Send exp_decompile message to a client
pub async fn send_decompile_script(
    client_id: &str,
    id: u32,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let msg = ServerMessage::ExpDecompile { id };
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize exp_decompile message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}
