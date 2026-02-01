//! WebSocket message handlers for different features
//!
//! This module contains handlers for processing incoming WebSocket messages
//! from clients and sending responses. Each submodule handles a specific feature:
//!
//! - `executor`: Script execution messages (ready, auto-execute)
//! - `explorer`: Instance explorer messages (tree, properties, search)
//! - `remote_spy`: Remote spy messages (calls, decompile, code generation)
//! - `logging`: Client log messages

pub mod executor;
pub mod explorer;
pub mod logging;
pub mod relay;
pub mod remote_spy;
