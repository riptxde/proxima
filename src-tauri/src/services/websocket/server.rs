use crate::state::{
    ActiveExplorerClient, ActiveRemoteSpyClient, ApiDumpCache, ClientInfo, ClientRegistry,
};
use crate::utils::events::emit_or_log;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;

use super::client_manager::emit_clients_update;
use super::handlers::{executor, explorer, logging, remote_spy};
use super::heartbeat::start_heartbeat_monitor;
use super::messages::ClientMessage;

/// Start the WebSocket server
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

/// Handle a single WebSocket client connection
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
    let client_id_shared = Arc::new(RwLock::new(None::<String>));
    let heartbeat_task = start_heartbeat_monitor(
        tx.clone(),
        Arc::clone(&missed_pings),
        Arc::clone(&client_id_shared),
        Arc::clone(&clients_clone),
        app_handle_clone.clone(),
    );

    // Handle incoming messages
    while let Some(result) = ws_receiver.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    if let Ok(text) = msg.to_text() {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(text) {
                            handle_message(
                                client_msg,
                                &mut client_id,
                                &client_id_shared,
                                &tx,
                                &clients_clone,
                                &app_handle_clone,
                                &missed_pings,
                            )
                            .await;
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

    // Client disconnected - cleanup
    handle_disconnect(
        client_id,
        &clients_clone,
        &app_handle_clone,
        &active_explorer,
        &active_remote_spy,
    )
    .await;

    send_task.abort();
    heartbeat_task.abort();
    Ok(())
}

/// Route incoming messages to appropriate handlers
async fn handle_message(
    msg: ClientMessage,
    client_id: &mut Option<String>,
    client_id_shared: &Arc<RwLock<Option<String>>>,
    tx: &mpsc::UnboundedSender<Message>,
    clients: &ClientRegistry,
    app_handle: &AppHandle,
    missed_pings: &Arc<RwLock<u32>>,
) {
    match msg {
        ClientMessage::Ready => {
            executor::handle_ready(tx, app_handle).await;
        }
        ClientMessage::Register { username } => {
            let id = Uuid::new_v4().to_string();

            let client_info = ClientInfo {
                username: username.clone(),
                sender: tx.clone(),
            };

            clients.write().await.insert(id.clone(), client_info);
            *client_id = Some(id.clone());
            *client_id_shared.write().await = Some(id.clone());

            // Log client registration
            log_ui!(app_handle, Success, "Client attached: {}", username);

            // Emit clients-update event with full list
            emit_clients_update(app_handle, clients).await;
        }
        ClientMessage::Pong => {
            if let Some(id) = client_id {
                log::debug!("Received pong from {}, resetting missed ping counter", id);
            }
            *missed_pings.write().await = 0;
        }
        ClientMessage::Log { level, message } => {
            // Validate level is 0-3
            if level > 3 {
                log::error!("Invalid log level from WebSocket client: {}", level);
            } else {
                logging::handle_log(app_handle, level, message);
            }
        }
        ClientMessage::ExpTree { nodes } => {
            explorer::handle_exp_tree(app_handle, nodes);
        }
        ClientMessage::ExpProperties {
            id,
            props,
            special_props,
        } => {
            explorer::handle_exp_properties(app_handle, id, props, special_props);
        }
        ClientMessage::ExpSearchResults {
            query,
            results,
            total,
            limited,
        } => {
            explorer::handle_exp_search_results(app_handle, query, results, total, limited);
        }
        ClientMessage::ExpTreeChanged => {
            explorer::handle_exp_tree_changed(app_handle);
        }
        ClientMessage::ExpDecompiled { id, source } => {
            explorer::handle_exp_decompiled(app_handle, id, source);
        }
        ClientMessage::RspyCall {
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
        } => {
            remote_spy::handle_rspy_call(
                app_handle,
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
            );
        }
        ClientMessage::RspyDecompiled {
            script_path,
            source,
        } => {
            remote_spy::handle_rspy_decompiled(app_handle, script_path, source);
        }
        ClientMessage::RspyGeneratedCode { call_id, code } => {
            remote_spy::handle_rspy_generated_code(app_handle, call_id, code);
        }
    }
}

/// Handle client disconnection and cleanup
async fn handle_disconnect(
    client_id: Option<String>,
    clients: &ClientRegistry,
    app_handle: &AppHandle,
    active_explorer: &ActiveExplorerClient,
    active_remote_spy: &ActiveRemoteSpyClient,
) {
    if let Some(id) = client_id {
        // Get username before removing from registry
        let username = clients
            .read()
            .await
            .get(&id)
            .map(|info| info.username.clone());

        // If this was the active explorer client, clean up explorer state
        {
            let mut active = active_explorer.write().await;
            if active.as_ref() == Some(&id) {
                *active = None;
                emit_or_log(app_handle, "explorer-stopped", ());
                log::info!("Active explorer client disconnected, clearing explorer state");
            }
        }

        // If this was the active remote spy client, clean up remote spy state
        {
            let mut active = active_remote_spy.write().await;
            if active.as_ref() == Some(&id) {
                *active = None;
                emit_or_log(app_handle, "remote-spy-stopped", ());
                log::info!("Active remote spy client disconnected, clearing remote spy state");
            }
        }

        clients.write().await.remove(&id);

        // Log client disconnection
        if let Some(username) = username {
            log_ui!(app_handle, Info, "Client disconnected: {}", username);
        }

        emit_clients_update(app_handle, clients).await;
    }
}
