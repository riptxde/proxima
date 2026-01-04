use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

use crate::state::{LauncherQueueRegistry, QueuedLauncher};

const LAUNCHER_WS_PORT: u16 = 11375;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherProgress {
    pub progress: u8,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LauncherMessage {
    Progress { progress: u8, status: String, error: Option<String> },
    QueueJoin { launcher_id: String },
}

/// Start the launcher WebSocket server
pub async fn start_launcher_websocket(app_handle: AppHandle) -> Result<(), String> {
    let addr = format!("127.0.0.1:{}", LAUNCHER_WS_PORT);
    let listener = TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("Failed to bind launcher WebSocket server: {}", e))?;

    log::info!("Launcher WebSocket server listening on {}", addr);

    let app_handle = Arc::new(RwLock::new(app_handle));

    tauri::async_runtime::spawn(async move {
        while let Ok((stream, addr)) = listener.accept().await {
            log::debug!("Launcher connection from: {}", addr);

            let app_handle = app_handle.clone();

            tauri::async_runtime::spawn(async move {
                if let Err(e) = handle_launcher_connection(stream, app_handle).await {
                    log::debug!("Launcher connection error: {}", e);
                }
            });
        }
    });

    Ok(())
}

async fn handle_launcher_connection(
    stream: tokio::net::TcpStream,
    app_handle: Arc<RwLock<AppHandle>>,
) -> Result<(), String> {
    let ws_stream = accept_async(stream)
        .await
        .map_err(|e| format!("WebSocket handshake failed: {}", e))?;

    let (mut write, mut read) = ws_stream.split();

    // Track the launcher ID for this connection (if it joins the queue)
    let mut current_launcher_id: Option<String> = None;

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(launcher_msg) = serde_json::from_str::<LauncherMessage>(&text) {
                    match launcher_msg {
                        LauncherMessage::Progress { progress, status, error } => {
                            let app = app_handle.read().await;

                            #[derive(Serialize, Clone)]
                            struct ProgressEvent {
                                progress: u8,
                                status: String,
                                #[serde(skip_serializing_if = "Option::is_none")]
                                error: Option<String>,
                            }

                            let event = ProgressEvent { progress, status, error };

                            if let Err(e) = app.emit("launcher-progress", event) {
                                log::error!("Failed to emit launcher progress: {}", e);
                            }
                        }
                        LauncherMessage::QueueJoin { launcher_id } => {
                            let app = app_handle.read().await;

                            // Track this launcher ID for the connection
                            current_launcher_id = Some(launcher_id.clone());

                            // Add launcher to queue registry
                            if let Some(queue_registry) = app.try_state::<LauncherQueueRegistry>() {
                                let mut registry = queue_registry.write().await;
                                registry.insert(launcher_id, QueuedLauncher {});
                                let count = registry.len() as u32;

                                #[derive(Serialize, Clone)]
                                struct QueueUpdateEvent {
                                    count: u32,
                                }

                                let event = QueueUpdateEvent { count };

                                if let Err(e) = app.emit("launcher-queue-update", event) {
                                    log::error!("Failed to emit launcher queue update: {}", e);
                                }
                            }
                        }

                    }
                }
            }
            Ok(Message::Close(_)) => {
                log::debug!("Launcher closed connection");
                break;
            }
            Ok(Message::Ping(data)) => {
                let _ = write.send(Message::Pong(data)).await;
            }
            Err(e) => {
                log::debug!("Launcher WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    // Connection closed - remove launcher from queue if it was queued
    if let Some(launcher_id) = current_launcher_id {
        let app = app_handle.read().await;
        if let Some(queue_registry) = app.try_state::<LauncherQueueRegistry>() {
            let mut registry = queue_registry.write().await;
            if registry.remove(&launcher_id).is_some() {
                let count = registry.len() as u32;

                log::debug!("Removed launcher from queue on disconnect: {}", launcher_id);

                #[derive(Serialize, Clone)]
                struct QueueUpdateEvent {
                    count: u32,
                }

                let event = QueueUpdateEvent { count };

                if let Err(e) = app.emit("launcher-queue-update", event) {
                    log::error!("Failed to emit launcher queue update: {}", e);
                }
            }
        }
    }

    Ok(())
}

use tokio::sync::Mutex as TokioMutex;
use tokio::runtime::Runtime;
use tokio_tungstenite::MaybeTlsStream;

type WsStream = tokio_tungstenite::WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

/// Global Tokio runtime for launcher (must persist for WebSocket to work)
static LAUNCHER_RUNTIME: once_cell::sync::Lazy<Runtime> =
    once_cell::sync::Lazy::new(|| Runtime::new().expect("Failed to create Tokio runtime"));

/// Global WebSocket connection for launcher (async-safe)
static LAUNCHER_CONNECTION: once_cell::sync::Lazy<TokioMutex<Option<WsStream>>> =
    once_cell::sync::Lazy::new(|| TokioMutex::new(None));

/// Establish persistent WebSocket connection (without joining queue)
pub fn connect() -> Result<(), String> {
    LAUNCHER_RUNTIME.block_on(async {
        let url = format!("ws://127.0.0.1:{}", LAUNCHER_WS_PORT);

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url)
            .await
            .map_err(|e| format!("Failed to connect to Proxima: {}", e))?;

        // Store connection globally
        *LAUNCHER_CONNECTION.lock().await = Some(ws_stream);

        Ok(())
    })
}

/// Send queue join message through existing connection
pub fn join_queue(launcher_id: String) -> Result<(), String> {
    LAUNCHER_RUNTIME.block_on(async {
        send_message_internal(LauncherMessage::QueueJoin { launcher_id }).await
    })
}

/// Send a message through the persistent connection
async fn send_message_internal(message: LauncherMessage) -> Result<(), String> {
    let mut conn_guard = LAUNCHER_CONNECTION.lock().await;

    if let Some(ws_stream) = conn_guard.as_mut() {
        let json = serde_json::to_string(&message)
            .map_err(|e| format!("Failed to serialize message: {}", e))?;

        ws_stream
            .send(Message::Text(json))
            .await
            .map_err(|e| format!("Failed to send message: {}", e))?;
    } else {
        return Err("No WebSocket connection established".to_string());
    }

    Ok(())
}

/// Send progress update through persistent connection
pub fn send_progress(progress: u8, status: &str) -> Result<(), String> {
    send_progress_with_error(progress, status, None)
}

/// Send progress update with optional error message through persistent connection
pub fn send_progress_with_error(progress: u8, status: &str, error: Option<String>) -> Result<(), String> {
    LAUNCHER_RUNTIME.block_on(async {
        let message = LauncherMessage::Progress {
            progress,
            status: status.to_string(),
            error,
        };
        send_message_internal(message).await
    })
}

/// Close the persistent connection (automatically removes from queue via disconnect handler)
pub fn disconnect() -> Result<(), String> {
    LAUNCHER_RUNTIME.block_on(async {
        let mut conn_guard = LAUNCHER_CONNECTION.lock().await;

        if let Some(mut ws_stream) = conn_guard.take() {
            let _ = ws_stream.close(None).await;
        }

        Ok(())
    })
}
