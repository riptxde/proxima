use crate::models::FileNode;
use crate::services::filesystem;
use tauri::AppHandle;

/// Read the file tree for scripts and autoexec directories
/// This is an async command to prevent blocking the UI when loading large file trees
#[tauri::command]
pub async fn read_file_tree(app: AppHandle) -> Result<Vec<FileNode>, String> {
    // Run the file tree building in a blocking task to avoid blocking the async runtime
    tauri::async_runtime::spawn_blocking(move || filesystem::build_file_tree(&app))
        .await
        .map_err(|e| format!("Failed to spawn file tree task: {}", e))?
}
