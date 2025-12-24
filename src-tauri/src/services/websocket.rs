use crate::models::explorer::*;
use crate::models::remote_spy::*;
use crate::models::Client;
use crate::services::autoexec;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::RwLock;
use tokio::time::interval;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;

// Custom deserializer for HashMap that handles empty arrays from Lua
fn deserialize_props<'de, D>(deserializer: D) -> Result<HashMap<String, PropertyData>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Object(map) => {
            serde_json::from_value(Value::Object(map)).map_err(serde::de::Error::custom)
        }
        Value::Array(arr) if arr.is_empty() => Ok(HashMap::new()),
        _ => Err(serde::de::Error::custom(
            "expected object or empty array for props",
        )),
    }
}

// Heartbeat configuration
const PING_INTERVAL: f64 = 5.0; // seconds
const MAX_MISSED_CYCLES: u32 = 2;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientMessage {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "register")]
    Register { username: String },
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "log")]
    Log { level: u8, message: String },
    #[serde(rename = "exp_tree")]
    ExpTree { nodes: Vec<ExplorerNode> },
    #[serde(rename = "exp_properties")]
    ExpProperties {
        id: u32,
        #[serde(deserialize_with = "deserialize_props")]
        props: HashMap<String, PropertyData>,
        #[serde(rename = "specialProps", deserialize_with = "deserialize_props")]
        special_props: HashMap<String, PropertyData>,
    },
    #[serde(rename = "exp_search_results")]
    ExpSearchResults {
        query: String,
        results: Vec<SearchResult>,
        total: u32,
        limited: bool,
    },
    #[serde(rename = "exp_tree_changed")]
    ExpTreeChanged,
    #[serde(rename = "exp_decompiled")]
    ExpDecompiled { id: u32, source: String },
    #[serde(rename = "rspy_call")]
    RspyCall {
        #[serde(rename = "remoteId")]
        remote_id: u32,
        name: String,
        path: String,
        #[serde(rename = "remoteType")]
        remote_type: String,
        direction: String,
        timestamp: String,
        arguments: Vec<RemoteArgument>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "returnValue")]
        return_value: Option<RemoteArgument>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "callingScript")]
        calling_script: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "callingScriptPath")]
        calling_script_path: Option<String>,
    },
    #[serde(rename = "rspy_decompiled")]
    RspyDecompiled {
        #[serde(rename = "scriptPath")]
        script_path: String,
        source: String,
    },
    #[serde(rename = "rspy_generated_code")]
    RspyGeneratedCode {
        #[serde(rename = "callId")]
        call_id: String,
        code: String,
    },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
enum ServerMessage {
    #[serde(rename = "exec")]
    Exec { script: String },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "exp_start")]
    ExpStart,
    #[serde(rename = "exp_stop")]
    ExpStop,
    #[serde(rename = "exp_decompile")]
    ExpDecompile { id: u32 },
    #[serde(rename = "rspy_start")]
    RspyStart,
    #[serde(rename = "rspy_stop")]
    RspyStop,
    #[serde(rename = "rspy_decompile")]
    RspyDecompile {
        #[serde(rename = "scriptPath")]
        script_path: String,
    },
    #[serde(rename = "rspy_generate_code")]
    RspyGenerateCode {
        #[serde(rename = "callId")]
        call_id: String,
        name: String,
        path: String,
        #[serde(rename = "remoteType")]
        remote_type: String,
        direction: String,
        arguments: Vec<RemoteArgument>,
    },
}

pub(crate) struct ClientInfo {
    pub(crate) username: String,
    pub(crate) sender: UnboundedSender<Message>,
}

pub type ClientRegistry = Arc<RwLock<HashMap<String, ClientInfo>>>;

// Explorer state
pub type ActiveExplorerClient = Arc<RwLock<Option<String>>>;
pub type ApiDumpCache = Arc<RwLock<super::api_dump::ApiDumpService>>;

// Remote Spy state
pub type ActiveRemoteSpyClient = Arc<RwLock<Option<String>>>;

pub async fn start_websocket_server(
    app_handle: AppHandle,
    clients: ClientRegistry,
    active_explorer: ActiveExplorerClient,
    active_remote_spy: ActiveRemoteSpyClient,
    api_dump_cache: ApiDumpCache,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:13376").await?;
    log_ui!(
        &app_handle,
        Success,
        "WebSocket server started on port 13376"
    );

    while let Ok((stream, addr)) = listener.accept().await {
        let clients = Arc::clone(&clients);
        let app_handle = app_handle.clone();
        let active_explorer = Arc::clone(&active_explorer);
        let active_remote_spy = Arc::clone(&active_remote_spy);
        let api_dump_cache = Arc::clone(&api_dump_cache);

        tauri::async_runtime::spawn(async move {
            if let Err(e) = handle_client(
                stream,
                addr,
                clients,
                app_handle,
                active_explorer,
                active_remote_spy,
                api_dump_cache,
            )
            .await
            {
                log::error!("Error handling client {}: {}", addr, e);
            }
        });
    }

    Ok(())
}

async fn handle_client(
    stream: TcpStream,
    addr: SocketAddr,
    clients: ClientRegistry,
    app_handle: AppHandle,
    active_explorer: ActiveExplorerClient,
    active_remote_spy: ActiveRemoteSpyClient,
    _api_dump_cache: ApiDumpCache,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    log::info!("WebSocket connection established: {}", addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Create channel for outgoing messages
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let mut client_id: Option<String> = None;
    let clients_clone = Arc::clone(&clients);
    let app_handle_clone = app_handle.clone();
    let missed_pings = Arc::new(RwLock::new(0u32));

    // Spawn task to handle outgoing messages
    let send_task = tauri::async_runtime::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Spawn heartbeat task
    let tx_heartbeat = tx.clone();
    let missed_pings_heartbeat = Arc::clone(&missed_pings);
    let clients_heartbeat = Arc::clone(&clients_clone);
    let app_handle_heartbeat = app_handle_clone.clone();
    let client_id_heartbeat = Arc::new(RwLock::new(None::<String>));
    let client_id_heartbeat_task = Arc::clone(&client_id_heartbeat);

    let heartbeat_task = tauri::async_runtime::spawn(async move {
        let mut interval = interval(Duration::from_secs_f64(PING_INTERVAL));
        interval.tick().await; // Skip first immediate tick

        loop {
            interval.tick().await;

            let client_id_opt = client_id_heartbeat_task.read().await.clone();
            let current_missed = *missed_pings_heartbeat.read().await;

            // Check missed pings
            if current_missed >= MAX_MISSED_CYCLES {
                if let Some(id) = &client_id_opt {
                    log::warn!(
                        "Client {} failed to respond after {} cycles, disconnecting",
                        id,
                        MAX_MISSED_CYCLES
                    );
                }
                break;
            }

            // Increment missed pings counter
            *missed_pings_heartbeat.write().await += 1;

            // Send ping
            let ping_msg = ServerMessage::Ping;
            let ping_text = match serde_json::to_string(&ping_msg) {
                Ok(text) => text,
                Err(_) => break,
            };

            if let Some(id) = &client_id_opt {
                log::debug!("Sending ping to {} (missed: {})", id, current_missed);
            }

            if tx_heartbeat.send(Message::Text(ping_text)).is_err() {
                if let Some(id) = &client_id_opt {
                    log::warn!("Failed to send ping to {}, connection likely closed", id);
                }
                break;
            }
        }

        // Cleanup on heartbeat failure - forcefully close connection
        if let Some(id) = client_id_heartbeat_task.read().await.as_ref() {
            clients_heartbeat.write().await.remove(id);
            log::info!("Removed dead client: {}", id);

            // Send close message to forcefully disconnect
            let _ = tx_heartbeat.send(Message::Close(None));

            emit_clients_update(&app_handle_heartbeat, &clients_heartbeat).await;
        }
    });

    // Handle incoming messages
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    if let Ok(text) = msg.to_text() {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(text) {
                            match client_msg {
                                ClientMessage::Ready => {
                                    log::info!("Client ready, sending auto-execute scripts");

                                    // Check if auto-execute is enabled
                                    let auto_execute =
                                        get_auto_execute_setting(&app_handle_clone).await;

                                    if auto_execute {
                                        // Get autoexec scripts
                                        let scripts =
                                            autoexec::get_autoexec_scripts(&app_handle_clone);

                                        if !scripts.is_empty() {
                                            let script_count = scripts.len();

                                            // Log autoexec
                                            let script_text = if script_count == 1 {
                                                "1 script".to_string()
                                            } else {
                                                format!("{} scripts", script_count)
                                            };
                                            log_ui!(
                                                &app_handle_clone,
                                                Success,
                                                "Auto-executing {} on new client",
                                                script_text
                                            );

                                            // Execute each script on this client
                                            for script in scripts {
                                                let execute_msg = ServerMessage::Exec { script };
                                                if let Ok(msg_text) =
                                                    serde_json::to_string(&execute_msg)
                                                {
                                                    if tx.send(Message::Text(msg_text)).is_err() {
                                                        log::error!(
                                                            "Failed to send autoexec script"
                                                        );
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                ClientMessage::Register { username } => {
                                    let id = Uuid::new_v4().to_string();

                                    let client_info = ClientInfo {
                                        username: username.clone(),
                                        sender: tx.clone(),
                                    };

                                    clients_clone.write().await.insert(id.clone(), client_info);
                                    client_id = Some(id.clone());
                                    *client_id_heartbeat.write().await = Some(id.clone());

                                    // Log client registration
                                    log_ui!(
                                        &app_handle_clone,
                                        Success,
                                        "Client attached: {}",
                                        username
                                    );

                                    // Emit clients-update event with full list
                                    emit_clients_update(&app_handle_clone, &clients_clone).await;
                                }
                                ClientMessage::Pong => {
                                    if let Some(id) = &client_id {
                                        log::debug!(
                                            "Received pong from {}, resetting missed ping counter",
                                            id
                                        );
                                    }
                                    *missed_pings.write().await = 0;
                                }
                                ClientMessage::Log { level, message } => {
                                    // Validate level is 0-3
                                    if level > 3 {
                                        log::error!(
                                            "Invalid log level from WebSocket client: {}",
                                            level
                                        );
                                    } else {
                                        // Emit log-message event to frontend
                                        #[derive(Serialize, Clone)]
                                        struct LogMessage {
                                            level: u8,
                                            message: String,
                                        }

                                        let log_msg = LogMessage { level, message };
                                        if let Err(e) =
                                            app_handle_clone.emit("log-message", log_msg)
                                        {
                                            log::error!(
                                                "Failed to emit log event from WebSocket: {}",
                                                e
                                            );
                                        }
                                    }
                                }
                                ClientMessage::ExpTree { nodes } => {
                                    if is_active_explorer(&client_id, &active_explorer).await {
                                        emit_explorer_event(
                                            &app_handle_clone,
                                            "explorer-tree",
                                            TreeEvent { nodes },
                                        );
                                    }
                                }
                                ClientMessage::ExpProperties {
                                    id,
                                    props,
                                    special_props,
                                } => {
                                    if is_active_explorer(&client_id, &active_explorer).await {
                                        emit_explorer_event(
                                            &app_handle_clone,
                                            "explorer-properties",
                                            PropertiesEvent {
                                                id,
                                                props,
                                                special_props,
                                            },
                                        );
                                    }
                                }
                                ClientMessage::ExpSearchResults {
                                    query,
                                    results,
                                    total,
                                    limited,
                                } => {
                                    if is_active_explorer(&client_id, &active_explorer).await {
                                        emit_explorer_event(
                                            &app_handle_clone,
                                            "explorer-search-results",
                                            SearchResultsEvent {
                                                query,
                                                results,
                                                total,
                                                limited,
                                            },
                                        );
                                    }
                                }
                                ClientMessage::ExpTreeChanged => {
                                    if is_active_explorer(&client_id, &active_explorer).await {
                                        emit_explorer_event(
                                            &app_handle_clone,
                                            "explorer-tree-changed",
                                            (),
                                        );
                                    }
                                }
                                ClientMessage::ExpDecompiled { id, source } => {
                                    if is_active_explorer(&client_id, &active_explorer).await {
                                        emit_explorer_event(
                                            &app_handle_clone,
                                            "explorer-decompiled-script",
                                            DecompiledScriptEvent { id, source },
                                        );
                                    }
                                }
                                ClientMessage::RspyCall {
                                    remote_id,
                                    name,
                                    path,
                                    remote_type,
                                    direction,
                                    timestamp,
                                    arguments,
                                    return_value,
                                    calling_script,
                                    calling_script_path,
                                } => {
                                    let _ = app_handle_clone.emit(
                                        "remote-spy-call",
                                        RemoteCallEvent {
                                            remote_id,
                                            name,
                                            path,
                                            remote_type,
                                            direction,
                                            timestamp,
                                            arguments,
                                            return_value,
                                            calling_script,
                                            calling_script_path,
                                        },
                                    );
                                }
                                ClientMessage::RspyDecompiled {
                                    script_path,
                                    source,
                                } => {
                                    let _ = app_handle_clone.emit(
                                        "remote-spy-decompiled",
                                        serde_json::json!({
                                            "scriptPath": script_path,
                                            "source": source,
                                        }),
                                    );
                                }
                                ClientMessage::RspyGeneratedCode { call_id, code } => {
                                    let _ = app_handle_clone.emit(
                                        "remote-spy-generated-code",
                                        serde_json::json!({
                                            "callId": call_id,
                                            "code": code,
                                        }),
                                    );
                                }
                            }
                        } else {
                            log::warn!("Failed to parse client message: {}", text);
                        }
                    }
                } else if msg.is_close() {
                    break;
                }
            }
            Err(e) => {
                log::error!("WebSocket error: {}", e);
                break;
            }
        }
    }

    // Client disconnected
    if let Some(id) = client_id {
        // Get username before removing from registry
        let username = clients_clone
            .read()
            .await
            .get(&id)
            .map(|info| info.username.clone());

        // If this was the active explorer client, clean up explorer state
        {
            let mut active = active_explorer.write().await;
            if active.as_ref() == Some(&id) {
                *active = None;
                if let Err(e) = app_handle_clone.emit("explorer-stopped", ()) {
                    log::error!("Failed to emit explorer-stopped event: {}", e);
                }
                log::info!("Active explorer client disconnected, clearing explorer state");
            }
        }

        // If this was the active remote spy client, clean up remote spy state
        {
            let mut active = active_remote_spy.write().await;
            if active.as_ref() == Some(&id) {
                *active = None;
                if let Err(e) = app_handle_clone.emit("remote-spy-stopped", ()) {
                    log::error!("Failed to emit remote-spy-stopped event: {}", e);
                }
                log::info!("Active remote spy client disconnected, clearing remote spy state");
            }
        }

        clients_clone.write().await.remove(&id);

        // Log client disconnection
        if let Some(username) = username {
            log_ui!(&app_handle_clone, Info, "Client disconnected: {}", username);
        }

        emit_clients_update(&app_handle_clone, &clients_clone).await;
    }

    send_task.abort();
    heartbeat_task.abort();
    Ok(())
}

async fn emit_clients_update(app_handle: &AppHandle, clients: &ClientRegistry) {
    let clients_list: Vec<Client> = clients
        .read()
        .await
        .iter()
        .map(|(id, info)| Client {
            id: id.clone(),
            username: info.username.clone(),
        })
        .collect();

    if let Err(e) = app_handle.emit("clients-update", clients_list) {
        log::error!("Failed to emit clients-update event: {}", e);
    }
}

/// Check if a client is the active explorer client
async fn is_active_explorer(
    client_id: &Option<String>,
    active_explorer: &ActiveExplorerClient,
) -> bool {
    if let Some(id) = client_id {
        let active = active_explorer.read().await;
        active.as_ref() == Some(id)
    } else {
        false
    }
}

/// Emit an explorer event to the frontend
fn emit_explorer_event<T: Serialize + Clone>(app_handle: &AppHandle, event_name: &str, payload: T) {
    if let Err(e) = app_handle.emit(event_name, payload) {
        log::error!("Failed to emit {} event: {}", event_name, e);
    }
}

pub async fn broadcast_to_clients(
    client_ids: Vec<String>,
    script: String,
    clients: ClientRegistry,
) -> Result<(), String> {
    log::info!("Broadcasting script to {} client(s)", client_ids.len());

    let execute_msg = ServerMessage::Exec { script };
    let message_text = serde_json::to_string(&execute_msg)
        .map_err(|e| format!("Failed to serialize exec message: {}", e))?;

    let clients_lock = clients.read().await;
    let mut executed_count = 0;

    for client_id in &client_ids {
        if let Some(client_info) = clients_lock.get(client_id) {
            if client_info
                .sender
                .send(Message::Text(message_text.clone()))
                .is_ok()
            {
                executed_count += 1;
                log::debug!("Sent to client: {}", client_id);
            } else {
                log::error!("Failed to send to client: {}", client_id);
            }
        } else {
            log::error!("Client not found: {}", client_id);
        }
    }

    log::info!(
        "Successfully sent to {}/{} clients",
        executed_count,
        client_ids.len()
    );

    Ok(())
}

pub async fn get_attached_clients(clients: ClientRegistry) -> Vec<Client> {
    clients
        .read()
        .await
        .iter()
        .map(|(id, info)| Client {
            id: id.clone(),
            username: info.username.clone(),
        })
        .collect()
}

/// Read the autoExecute setting from the Tauri store
async fn get_auto_execute_setting(app: &AppHandle) -> bool {
    use crate::utils::paths;
    use tauri_plugin_store::StoreExt;

    // Get the base directory (same as scripts/autoexec location)
    let base_dir = match paths::get_base_directory(app) {
        Ok(dir) => dir,
        Err(e) => {
            log::error!("Failed to get base directory: {}", e);
            return true; // Default to true on error
        }
    };

    let settings_path = base_dir.join("settings.json");
    let settings_path_str = settings_path.to_string_lossy().to_string();

    match app.store(&settings_path_str) {
        Ok(store) => match store.get("settings") {
            Some(Value::Object(settings)) => {
                if let Some(Value::Object(execution)) = settings.get("execution") {
                    if let Some(Value::Bool(auto_execute)) = execution.get("autoExecute") {
                        return *auto_execute;
                    }
                }
            }
            _ => {}
        },
        Err(e) => {
            log::error!("Failed to access settings store: {}", e);
        }
    }

    // Default to true if setting not found
    true
}

/// Send a message to a specific client
pub async fn send_to_client(
    client_id: &str,
    message: &str,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let clients_lock = clients.read().await;

    if let Some(client_info) = clients_lock.get(client_id) {
        client_info
            .sender
            .send(Message::Text(message.to_string()))
            .map_err(|e| format!("Failed to send message to client: {}", e))?;
        Ok(())
    } else {
        Err(format!("Client not found: {}", client_id))
    }
}

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

// Remote Spy helper functions

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
    call_id: String,
    name: String,
    path: String,
    remote_type: String,
    direction: String,
    arguments: Vec<RemoteArgument>,
    clients: &ClientRegistry,
) -> Result<(), String> {
    let msg = ServerMessage::RspyGenerateCode {
        call_id,
        name,
        path,
        remote_type,
        direction,
        arguments,
    };
    let msg_text = serde_json::to_string(&msg)
        .map_err(|e| format!("Failed to serialize rspy_generate_code message: {}", e))?;

    send_to_client(client_id, &msg_text, clients).await
}
