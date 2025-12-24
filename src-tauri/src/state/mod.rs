use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

/// Client information stored in the registry
///
/// Contains the username and message sender for each connected WebSocket client.
pub struct ClientInfo {
    pub username: String,
    pub sender: UnboundedSender<Message>,
}

/// Registry of all connected WebSocket clients
///
/// Maps client ID (UUID string) to client information.
/// Shared across all features (executor, explorer, remote spy).
pub type ClientRegistry = Arc<RwLock<HashMap<String, ClientInfo>>>;

/// Active explorer client
///
/// Stores the client ID of the currently active explorer session.
/// Only one client can have explorer active at a time.
pub type ActiveExplorerClient = Arc<RwLock<Option<String>>>;

/// API dump cache
///
/// Cached Roblox API dump service for property metadata.
/// Shared across explorer features.
pub type ApiDumpCache = Arc<RwLock<crate::services::api_dump::ApiDumpService>>;

/// Active remote spy client
///
/// Stores the client ID of the currently active remote spy session.
/// Only one client can have remote spy active at a time.
pub type ActiveRemoteSpyClient = Arc<RwLock<Option<String>>>;
