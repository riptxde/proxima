use std::path::PathBuf;
use tauri::AppHandle;

/// Get the base directory for storing scripts and files
pub fn get_base_directory(_app: &AppHandle) -> Result<PathBuf, String> {
    if cfg!(debug_assertions) {
        // Development mode: use @dev folder in project root
        // Since current_dir is src-tauri during dev, we need to go up one level
        let current_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {}", e))?;

        // Go up one directory from src-tauri to project root
        let project_root = current_dir
            .parent()
            .ok_or_else(|| "Failed to get parent directory".to_string())?;

        Ok(project_root.join("@dev"))
    } else {
        // Production mode: use directory where the executable is located
        let exe_path =
            std::env::current_exe().map_err(|e| format!("Failed to get executable path: {}", e))?;
        let exe_dir = exe_path
            .parent()
            .ok_or_else(|| "Failed to get executable directory".to_string())?;
        Ok(exe_dir.to_path_buf())
    }
}
