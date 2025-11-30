use crate::models::FileNode;
use crate::services::filesystem;
use tauri::AppHandle;

/// Read the file tree for Scripts and AutoExec directories
#[tauri::command]
pub fn read_file_tree(app: AppHandle) -> Result<Vec<FileNode>, String> {
    filesystem::build_file_tree(&app)
}
