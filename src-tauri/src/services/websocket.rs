use crate::models::Client;
use crate::services::autoexec;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
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

// Heartbeat configuration
const PING_INTERVAL: f64 = 5.0; // seconds
const MAX_MISSED_CYCLES: u32 = 2;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientMessage {
    #[serde(rename = "register")]
    Register { username: String },
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "log")]
    Log { level: u8, message: String },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
enum ServerMessage {
    #[serde(rename = "execute")]
    Execute { script: String },
    #[serde(rename = "ping")]
    Ping,
}

pub(crate) struct ClientInfo {
    pub(crate) username: String,
    pub(crate) sender: UnboundedSender<Message>,
}

pub type ClientRegistry = Arc<RwLock<HashMap<String, ClientInfo>>>;

pub async fn start_websocket_server(
    app_handle: AppHandle,
    clients: ClientRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:13376").await?;
    println!("WebSocket server listening on ws://127.0.0.1:13376");

    // Log server start
    let _ = app_handle.emit("log-message", serde_json::json!({
        "level": 1,
        "message": "WebSocket server started on port 13376"
    }));

    while let Ok((stream, addr)) = listener.accept().await {
        let clients = Arc::clone(&clients);
        let app_handle = app_handle.clone();

        tauri::async_runtime::spawn(async move {
            if let Err(e) = handle_client(stream, addr, clients, app_handle).await {
                eprintln!("Error handling client {}: {}", addr, e);
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
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established: {}", addr);

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
                    println!(
                        "Client {} failed to respond after {} cycles, disconnecting",
                        id, MAX_MISSED_CYCLES
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
                println!("Sending ping to {} (missed: {})", id, current_missed);
            }

            if tx_heartbeat.send(Message::Text(ping_text)).is_err() {
                if let Some(id) = &client_id_opt {
                    println!("Failed to send ping to {}, connection likely closed", id);
                }
                break;
            }
        }

        // Cleanup on heartbeat failure - forcefully close connection
        if let Some(id) = client_id_heartbeat_task.read().await.as_ref() {
            clients_heartbeat.write().await.remove(id);
            println!("Removed dead client: {}", id);

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
                                ClientMessage::Register { username } => {
                                    let id = Uuid::new_v4().to_string();
                                    println!("Client registered: {} ({})", username, id);

                                    let client_info = ClientInfo {
                                        username: username.clone(),
                                        sender: tx.clone(),
                                    };

                                    clients_clone.write().await.insert(id.clone(), client_info);
                                    client_id = Some(id.clone());
                                    *client_id_heartbeat.write().await = Some(id.clone());

                                    // Log client registration
                                    let _ = app_handle_clone.emit("log-message", serde_json::json!({
                                        "level": 1,
                                        "message": format!("Client attached: {}", username)
                                    }));

                                    // Check if auto-execute is enabled
                                    let auto_execute = get_auto_execute_setting(&app_handle_clone).await;

                                    if auto_execute {
                                        // Get AutoExec scripts
                                        let scripts = autoexec::get_autoexec_scripts(&app_handle_clone);

                                        if !scripts.is_empty() {
                                            let script_count = scripts.len();
                                            println!("Auto-executing {} script(s) for client {}", script_count, id);

                                            // Log autoexec
                                            let script_text = if script_count == 1 {
                                                "1 script".to_string()
                                            } else {
                                                format!("{} scripts", script_count)
                                            };
                                            let _ = app_handle_clone.emit("log-message", serde_json::json!({
                                                "level": 1,
                                                "message": format!("Auto-executed {} on {}", script_text, username)
                                            }));

                                            // Execute each script on this client
                                            for script in scripts {
                                                let execute_msg = ServerMessage::Execute { script };
                                                if let Ok(msg_text) = serde_json::to_string(&execute_msg) {
                                                    if tx.send(Message::Text(msg_text)).is_err() {
                                                        eprintln!("Failed to send AutoExec script to client {}", id);
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Emit clients-update event with full list
                                    emit_clients_update(&app_handle_clone, &clients_clone).await;
                                }
                                ClientMessage::Pong => {
                                    if let Some(id) = &client_id {
                                        println!(
                                            "Received pong from {}, resetting missed ping counter",
                                            id
                                        );
                                    }
                                    *missed_pings.write().await = 0;
                                }
                                ClientMessage::Log { level, message } => {
                                    // Validate level is 0-3
                                    if level > 3 {
                                        eprintln!("Invalid log level from WebSocket client: {}", level);
                                    } else {
                                        // Emit log-message event to frontend
                                        #[derive(Serialize, Clone)]
                                        struct LogMessage {
                                            level: u8,
                                            message: String,
                                        }

                                        let log_msg = LogMessage { level, message };
                                        if let Err(e) = app_handle_clone.emit("log-message", log_msg) {
                                            eprintln!("Failed to emit log event from WebSocket: {}", e);
                                        }
                                    }
                                }
                            }
                        } else {
                            eprintln!("Failed to parse client message: {}", text);
                        }
                    }
                } else if msg.is_close() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }

    // Client disconnected
    if let Some(id) = client_id {
        // Get username before removing from registry
        let username = clients_clone.read().await.get(&id).map(|info| info.username.clone());

        clients_clone.write().await.remove(&id);
        println!("Client disconnected: {}", id);

        // Log client disconnection
        if let Some(username) = username {
            let _ = app_handle_clone.emit("log-message", serde_json::json!({
                "level": 0,
                "message": format!("Client disconnected: {}", username)
            }));
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
        eprintln!("Failed to emit clients-update event: {}", e);
    }
}

pub async fn broadcast_to_clients(
    client_ids: Vec<String>,
    script: String,
    clients: ClientRegistry,
) -> Result<(), String> {
    println!("Broadcasting script to {} client(s)", client_ids.len());

    let execute_msg = ServerMessage::Execute { script };
    let message_text = serde_json::to_string(&execute_msg)
        .map_err(|e| format!("Failed to serialize execute message: {}", e))?;

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
                println!("Sent to client: {}", client_id);
            } else {
                eprintln!("Failed to send to client: {}", client_id);
            }
        } else {
            eprintln!("Client not found: {}", client_id);
        }
    }

    println!(
        "Successfully sent to {}/{} clients",
        executed_count,
        client_ids.len()
    );

    Ok(())
}

pub async fn get_connected_clients(clients: ClientRegistry) -> Vec<Client> {
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
    use tauri_plugin_store::StoreExt;

    match app.store("settings.json") {
        Ok(store) => {
            match store.get("settings") {
                Some(Value::Object(settings)) => {
                    if let Some(Value::Object(execution)) = settings.get("execution") {
                        if let Some(Value::Bool(auto_execute)) = execution.get("autoExecute") {
                            return *auto_execute;
                        }
                    }
                }
                _ => {}
            }
        }
        Err(e) => {
            eprintln!("Failed to access settings store: {}", e);
        }
    }

    // Default to true if setting not found
    true
}
