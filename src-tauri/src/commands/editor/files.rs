use crate::log_ui;
use crate::services::filesystem;
use crate::utils::paths;
use std::fs;
use tauri::{AppHandle, Emitter};

/// Get the base scripts path
#[tauri::command]
pub fn get_scripts_path(app: AppHandle) -> Result<String, String> {
    let base_dir = paths::get_base_directory(&app)?;
    Ok(base_dir.to_string_lossy().to_string())
}

/// Initialize scripts and autoexec directories
#[tauri::command]
pub fn initialize_directories(app: AppHandle) -> Result<(), String> {
    filesystem::initialize_directories(&app)
}

/// Read the content of a file by relative path
#[tauri::command]
pub fn read_file_content(app: AppHandle, relative_path: String) -> Result<String, String> {
    let base_dir = paths::get_base_directory(&app)?;
    let file_path = base_dir.join(relative_path);

    fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))
}

/// Save a file to scripts or autoexec folder or any descendant path
#[tauri::command]
pub fn save_file(
    app: AppHandle,
    filename: String,
    folder: String,
    content: String,
) -> Result<String, String> {
    let base_dir = paths::get_base_directory(&app)?;

    // Build the target folder path
    let folder_path = base_dir.join(&folder);

    // Create the directory structure if it doesn't exist
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let file_path = folder_path.join(&filename);

    // Write the file
    fs::write(&file_path, content).map_err(|e| format!("Failed to write file: {}", e))?;

    // Return the relative path with forward slashes
    let relative_path = file_path
        .strip_prefix(&base_dir)
        .map_err(|e| format!("Failed to get relative path: {}", e))?
        .to_string_lossy()
        .replace('\\', "/");

    // Log successful file save
    log_ui!(&app, Success, "File saved: {}", relative_path);

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

    // Build the new path (same parent directory, new name)
    let parent = old_path
        .parent()
        .ok_or_else(|| "Invalid file path".to_string())?;
    let new_path = parent.join(&new_name);

    // Check if the old path exists
    if !old_path.exists() {
        return Err(format!("File or folder not found: {}", relative_path));
    }

    // Check if a file/folder with the new name already exists
    if new_path.exists() {
        return Err(format!(
            "A file or folder named '{}' already exists",
            new_name
        ));
    }

    // Perform the rename
    fs::rename(&old_path, &new_path).map_err(|e| format!("Failed to rename: {}", e))?;

    // Get the new relative path
    let new_relative_path = new_path
        .strip_prefix(&base_dir)
        .map_err(|e| format!("Failed to get relative path: {}", e))?
        .to_string_lossy()
        .replace('\\', "/");

    // Emit file tree changed event
    let _ = app.emit("file-tree-changed", ());

    // Log the rename operation
    log_ui!(
        &app,
        Success,
        "Renamed: {} -> {}",
        relative_path,
        new_relative_path
    );

    Ok(new_relative_path)
}

/// Delete a file or folder
#[tauri::command]
pub fn delete_file(app: AppHandle, relative_path: String, is_folder: bool) -> Result<(), String> {
    let base_dir = paths::get_base_directory(&app)?;
    let file_path = base_dir.join(&relative_path);

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
    log_ui!(&app, Success, "Deleted {}: {}", item_type, relative_path);

    Ok(())
}

/// Open the file location in the system file explorer
#[tauri::command]
pub fn open_file_location(app: AppHandle, relative_path: String) -> Result<(), String> {
    let base_dir = paths::get_base_directory(&app)?;

    // Normalize path separators to backslashes on Windows
    let normalized_path = relative_path.replace('/', "\\");
    let file_path = base_dir.join(&normalized_path);

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
    log_ui!(
        &app,
        Info,
        "Opened file/folder location: {}",
        parent_dir.display()
    );

    Ok(())
}
