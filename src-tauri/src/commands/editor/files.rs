use crate::services::filesystem;
use crate::utils::{paths, security};
use std::fs;
use tauri::{AppHandle, Emitter};

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

    // Log successful file save
    let _ = app.emit(
        "log-message",
        serde_json::json!({
            "level": 1,
            "message": format!("File saved: {}", relative_path)
        }),
    );

    Ok(relative_path)
}

/// Rename a file or folder
#[tauri::command]
pub fn rename_file(
    app: AppHandle,
    relative_path: String,
    new_name: String,
) -> Result<String, String> {
    let base_dir = paths::get_base_directory(&app)?;
    let old_path = base_dir.join(&relative_path);

    // Security check: ensure the old path is within the base directory
    security::validate_path(&old_path, &base_dir)?;

    // Sanitize the new name to prevent path traversal
    let safe_new_name = security::sanitize_filename(&new_name)?;

    // Build the new path (same parent directory, new name)
    let parent = old_path
        .parent()
        .ok_or_else(|| "Invalid file path".to_string())?;
    let new_path = parent.join(&safe_new_name);

    // Check if the old path exists
    if !old_path.exists() {
        return Err(format!("File or folder not found: {}", relative_path));
    }

    // Check if a file/folder with the new name already exists
    if new_path.exists() {
        return Err(format!(
            "A file or folder named '{}' already exists",
            safe_new_name
        ));
    }

    // Perform the rename
    fs::rename(&old_path, &new_path).map_err(|e| format!("Failed to rename: {}", e))?;

    // Security validation: ensure the new path is still within Scripts or AutoExec
    security::validate_scripts_path(&new_path, &base_dir)?;

    // Get the new relative path
    let new_relative_path = new_path
        .strip_prefix(&base_dir)
        .map_err(|e| format!("Failed to get relative path: {}", e))?
        .to_string_lossy()
        .replace('\\', "/");

    // Emit file tree changed event
    let _ = app.emit("file-tree-changed", ());

    // Log the rename operation
    let _ = app.emit(
        "log-message",
        serde_json::json!({
            "level": 1,
            "message": format!("Renamed: {} → {}", relative_path, new_relative_path)
        }),
    );

    Ok(new_relative_path)
}

/// Delete a file or folder
#[tauri::command]
pub fn delete_file(app: AppHandle, relative_path: String, is_folder: bool) -> Result<(), String> {
    let base_dir = paths::get_base_directory(&app)?;
    let file_path = base_dir.join(&relative_path);

    // Security check: ensure the path is within the base directory
    security::validate_path(&file_path, &base_dir)?;

    // Check if the path exists
    if !file_path.exists() {
        return Err(format!("File or folder not found: {}", relative_path));
    }

    // Delete the file or folder
    if is_folder {
        fs::remove_dir_all(&file_path).map_err(|e| format!("Failed to delete folder: {}", e))?;
    } else {
        fs::remove_file(&file_path).map_err(|e| format!("Failed to delete file: {}", e))?;
    }

    // Emit file tree changed event
    let _ = app.emit("file-tree-changed", ());

    // Log the delete operation
    let item_type = if is_folder { "folder" } else { "file" };
    let _ = app.emit(
        "log-message",
        serde_json::json!({
            "level": 1,
            "message": format!("Deleted {}: {}", item_type, relative_path)
        }),
    );

    Ok(())
}

/// Open the file location in the system file explorer
#[tauri::command]
pub fn open_file_location(app: AppHandle, relative_path: String) -> Result<(), String> {
    let base_dir = paths::get_base_directory(&app)?;

    // Normalize path separators to backslashes on Windows
    let normalized_path = relative_path.replace('/', "\\");
    let file_path = base_dir.join(&normalized_path);

    // Security check: ensure the path is within the base directory
    security::validate_path(&file_path, &base_dir)?;

    // Get the parent directory
    let parent_dir = if file_path.is_file() {
        file_path
            .parent()
            .ok_or_else(|| "Invalid file path".to_string())?
    } else {
        &file_path
    };

    // Open the parent directory using tauri-plugin-opener
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("explorer")
            .arg(parent_dir)
            .spawn()
            .map_err(|e| format!("Failed to open file location: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        return Err("Open file location is only supported on Windows".to_string());
    }

    // Log the operation
    let _ = app.emit(
        "log-message",
        serde_json::json!({
            "level": 0,
            "message": format!("Opened file/folder location: {}", parent_dir.display())
        }),
    );

    Ok(())
}
