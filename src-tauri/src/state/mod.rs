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

/// API dump cache
///
/// Cached Roblox API dump service for property metadata.
/// Shared across explorer features.
pub type ApiDumpCache = Arc<RwLock<crate::services::api_dump::ApiDumpService>>;

/// Active feature clients
///
/// Stores which client (if any) is currently using each feature.
/// The same client can use multiple features simultaneously.
pub struct ActiveClients {
    pub explorer: Option<String>,
    pub remote_spy: Option<String>,
}

pub type ActiveClientsState = Arc<RwLock<ActiveClients>>;
