#[macro_use]
mod utils;

mod commands;
mod models;
mod services;

use commands::editor::{
    delete_file, get_scripts_path, initialize_directories, open_file_location, read_file_content,
    read_file_tree, rename_file, save_file,
};
use commands::executor::{execute_script, get_attached_clients};
use commands::explorer::{
    explorer_get_properties, explorer_get_tree, explorer_search, start_explorer, stop_explorer,
};
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
            // Always initialize the logger for consistent timestamped logging
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(log::LevelFilter::Info)
                    .build(),
            )?;

            // Initialize client registry
            let clients: services::websocket::ClientRegistry =
                Arc::new(RwLock::new(HashMap::new()));
            app.manage(clients.clone());

            // Initialize explorer state
            let active_explorer: services::websocket::ActiveExplorerClient =
                Arc::new(RwLock::new(None));
            app.manage(active_explorer.clone());

            let api_dump_cache: services::websocket::ApiDumpCache =
                Arc::new(RwLock::new(services::api_dump::ApiDumpService::new()));
            app.manage(api_dump_cache.clone());

            // Load API dump in background
            let api_dump_clone = api_dump_cache.clone();
            tauri::async_runtime::spawn(async move {
                let mut service = api_dump_clone.write().await;
                if let Err(e) = service.load().await {
                    log::error!("Failed to load API dump: {}", e);
                } else {
                    log::info!("API dump loaded successfully");
                }
            });

            // Start the WebSocket server
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = services::websocket::start_websocket_server(
                    app_handle.clone(),
                    clients,
                    active_explorer,
                    api_dump_cache,
                )
                .await
                {
                    log::error!("Failed to start WebSocket server: {}", e);
                    log_ui!(
                        &app_handle,
                        Error,
                        "Failed to start WebSocket server: {}",
                        e
                    );
                }
            });

            // Start the file watcher
            let app_handle = app.handle().clone();
            if let Err(e) = services::file_watcher::start_file_watcher(app_handle.clone()) {
                log::error!("Failed to start file watcher: {}", e);
                log_ui!(&app_handle, Error, "Failed to start file watcher: {}", e);
            }

            // Start the HTTP server
            let app_handle_http = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) =
                    services::http_server::start_http_server(app_handle_http.clone()).await
                {
                    log::error!("Failed to start HTTP server: {}", e);
                    log_ui!(
                        &app_handle_http,
                        Error,
                        "Failed to start HTTP server: {}",
                        e
                    );
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Editor commands
            get_scripts_path,
            initialize_directories,
            read_file_tree,
            read_file_content,
            save_file,
            rename_file,
            delete_file,
            open_file_location,
            // Executor commands
            execute_script,
            get_attached_clients,
            // Explorer commands
            start_explorer,
            stop_explorer,
            explorer_get_tree,
            explorer_get_properties,
            explorer_search,
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
