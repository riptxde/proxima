//! WebSocket server for handling connections from Roblox executor clients
//!
//! This module provides a WebSocket server that enables communication between
//! Proxima and Roblox executor clients. It handles:
//!
//! - Script execution broadcasting
//! - Client registration and management
//! - Explorer instance tree browsing
//! - Remote spy call monitoring
//! - Heartbeat monitoring for connection health
//!
//! ## Architecture
//!
//! The WebSocket service is organized into focused submodules:
//!
//! - `server`: WebSocket server setup and client connection handling
//! - `client_manager`: Client registry operations and event emission
//! - `heartbeat`: Heartbeat monitoring for connection health
//! - `messages`: Message type definitions and serialization
//! - `handlers`: Feature-specific message processing
//!   - `executor`: Script execution and auto-execute
//!   - `explorer`: Instance tree exploration
//!   - `remote_spy`: Remote function/event monitoring
//!   - `logging`: Client log message forwarding
//!
//! ## Usage
//!
//! The server is started automatically when the Tauri app initializes:
//!
//! ```rust
//! use crate::services::websocket::start_websocket_server;
//!
//! tauri::async_runtime::spawn(async move {
//!     if let Err(e) = start_websocket_server(
//!         app_handle,
//!         clients,
//!         active_explorer,
//!         active_remote_spy,
//!         api_dump_cache,
//!     ).await {
//!         log::error!("WebSocket server error: {}", e);
//!     }
//! });
//! ```

mod client_manager;
mod heartbeat;
mod messages;
mod server;

pub mod handlers;

// Re-export public API
pub use client_manager::{broadcast_to_clients, get_attached_clients};
pub use handlers::explorer::{
    send_decompile_script, send_get_explorer_properties, send_get_explorer_tree,
    send_search_explorer, send_start_explorer, send_stop_explorer,
};
pub use handlers::remote_spy::{
    send_decompile_request, send_generate_code_request, send_start_remote_spy, send_stop_remote_spy,
};
pub use server::start_websocket_server;
