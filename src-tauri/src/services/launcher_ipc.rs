use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

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

    Ok(())
}

/// Send progress update from launcher binary to main Proxima app via WebSocket
pub fn send_progress(progress: u8, status: &str) -> Result<(), String> {
    send_progress_with_error(progress, status, None)
}

/// Send progress update with optional error message
pub fn send_progress_with_error(progress: u8, status: &str, error: Option<String>) -> Result<(), String> {
    // Use blocking runtime for the launcher binary
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| format!("Failed to create runtime: {}", e))?;

    rt.block_on(async {
        send_progress_async(progress, status, error).await
    })
}

async fn send_progress_async(progress: u8, status: &str, error: Option<String>) -> Result<(), String> {
    let url = format!("ws://127.0.0.1:{}", LAUNCHER_WS_PORT);

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(&url)
        .await
        .map_err(|_| {
            // Proxima might not be running, silently fail
            return String::new();
        })?;

    let message = LauncherMessage::Progress {
        progress,
        status: status.to_string(),
        error,
    };

    let json = serde_json::to_string(&message)
        .map_err(|e| format!("Failed to serialize message: {}", e))?;

    ws_stream
        .send(Message::Text(json))
        .await
        .map_err(|e| format!("Failed to send message: {}", e))?;

    // Close connection
    let _ = ws_stream.close(None).await;

    Ok(())
}
