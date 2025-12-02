mod commands;
mod models;
mod services;

use commands::editor::{
    get_scripts_path, initialize_directories, read_file_content, read_file_tree, save_file,
};
use commands::executor::{execute_script, get_attached_clients};
use commands::logs::add_log;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::RwLock;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize client registry
            let clients: services::websocket::ClientRegistry =
                Arc::new(RwLock::new(HashMap::new()));
            app.manage(clients.clone());

            // Start the WebSocket server
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) =
                    services::websocket::start_websocket_server(app_handle, clients).await
                {
                    eprintln!("Failed to start WebSocket server: {}", e);
                }
            });

            // Start the file watcher
            let app_handle = app.handle().clone();
            if let Err(e) = services::file_watcher::start_file_watcher(app_handle) {
                eprintln!("Failed to start file watcher: {}", e);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Editor commands
            get_scripts_path,
            initialize_directories,
            read_file_tree,
            read_file_content,
            save_file,
            // Executor commands
            execute_script,
            get_attached_clients,
            // Logs commands
            add_log,
            // Script Hub commands (future)
            // fetch_scripts,
            // Settings commands (future)
            // get_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
