// Launcher services module
//
// Provides modular launcher functionality:
// - IPC: WebSocket server for launcher communication
// - Paths: Development and production path resolution
// - Settings: Launcher configuration loading
// - Version: Roblox version fetching and validation
// - Installation: Package download and extraction
// - Mutex: Windows named mutex management (queue and multi-instance)
// - Process: Roblox process launching and monitoring

pub mod installation;
pub mod ipc;
pub mod mutex;
pub mod paths;
pub mod process;
pub mod settings;
pub mod version;

// Re-export commonly used types
pub use ipc::start_launcher_websocket;
pub use paths::LauncherPaths;
pub use settings::LauncherSettings;
