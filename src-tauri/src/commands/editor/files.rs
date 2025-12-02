use crate::services::{filesystem, paths, security};
use std::fs;
use tauri::AppHandle;

/// Get the base scripts path
#[tauri::command]
pub fn get_scripts_path(app: AppHandle) -> Result<String, String> {
    let base_dir = paths::get_base_directory(&app)?;
    Ok(base_dir.to_string_lossy().to_string())
}

/// Initialize Scripts and AutoExec directories
#[tauri::command]
pub fn initialize_directories(app: AppHandle) -> Result<(), String> {
    filesystem::initialize_directories(&app)
}

/// Read the content of a file by relative path
#[tauri::command]
pub fn read_file_content(app: AppHandle, relative_path: String) -> Result<String, String> {
    let base_dir = paths::get_base_directory(&app)?;
    let file_path = base_dir.join(relative_path);

    // Security check: ensure the file is within the base directory
    security::validate_path(&file_path, &base_dir)?;

    fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))
}

/// Save a file to Scripts or AutoExec folder or any descendant path
#[tauri::command]
pub fn save_file(
    app: AppHandle,
    filename: String,
    folder: String,
    content: String,
) -> Result<String, String> {
    let base_dir = paths::get_base_directory(&app)?;

    // Validate folder is within Scripts or AutoExec before creating anything
    security::validate_scripts_folder(&folder)?;

    // Build the target folder path
    let folder_path = base_dir.join(&folder);

    // Create the directory structure if it doesn't exist
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Sanitize filename to prevent path traversal
    let safe_filename = security::sanitize_filename(&filename)?;
    let file_path = folder_path.join(&safe_filename);

    // Write the file first (we need it to exist for canonicalization)
    fs::write(&file_path, content).map_err(|e| format!("Failed to write file: {}", e))?;

    // Security validation: ensure the final path is within Scripts or AutoExec using canonicalization
    if let Err(e) = security::validate_scripts_path(&file_path, &base_dir) {
        // Remove the file we just wrote since it's in an invalid location
        let _ = fs::remove_file(&file_path);
        return Err(e);
    }

    // Return the relative path with forward slashes
    let relative_path = file_path
        .strip_prefix(&base_dir)
        .map_err(|e| format!("Failed to get relative path: {}", e))?
        .to_string_lossy()
        .replace('\\', "/");

    Ok(relative_path)
}
