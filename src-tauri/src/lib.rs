mod commands;
mod models;
mod services;

use commands::editor::{
    get_scripts_path, initialize_directories, read_file_content, read_file_tree, save_file,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

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
            // Script Hub commands (future)
            // fetch_scripts,
            // Logs commands (future)
            // get_logs,
            // Settings commands (future)
            // get_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
