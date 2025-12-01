use crate::models::Client;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tauri::{AppHandle, Emitter};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientMessage {
    #[serde(rename = "register")]
    Register { username: String },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
enum ServerMessage {
    #[serde(rename = "execute")]
    Execute { script: String },
}

pub(crate) struct ClientInfo {
    pub(crate) username: String,
    pub(crate) sender: UnboundedSender<Message>,
    #[allow(dead_code)]
    pub(crate) connected_at: Instant,
}

pub type ClientRegistry = Arc<RwLock<HashMap<String, ClientInfo>>>;

pub async fn start_websocket_server(
    app_handle: AppHandle,
    clients: ClientRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:13376").await?;
    println!("WebSocket server listening on ws://127.0.0.1:13376");

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

    // Spawn task to handle outgoing messages
    let send_task = tauri::async_runtime::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
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
                                        connected_at: Instant::now(),
                                    };

                                    clients_clone.write().await.insert(id.clone(), client_info);
                                    client_id = Some(id.clone());

                                    // Emit clients-update event with full list
                                    emit_clients_update(&app_handle_clone, &clients_clone).await;
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
        clients_clone.write().await.remove(&id);
        println!("Client disconnected: {}", id);
        emit_clients_update(&app_handle_clone, &clients_clone).await;
    }

    send_task.abort();
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
