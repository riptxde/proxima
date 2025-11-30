use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FileNode {
    #[serde(rename = "file")]
    File {
        id: String,
        name: String,
        path: String, // Relative path from base directory
    },
    #[serde(rename = "folder")]
    Folder {
        id: String,
        name: String,
        children: Vec<FileNode>,
    },
}

fn get_base_directory(_app: &AppHandle) -> Result<PathBuf, String> {
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

#[tauri::command]
fn get_scripts_path(app: AppHandle) -> Result<String, String> {
    let base_dir = get_base_directory(&app)?;
    Ok(base_dir.to_string_lossy().to_string())
}

#[tauri::command]
fn initialize_directories(app: AppHandle) -> Result<(), String> {
    let base_dir = get_base_directory(&app)?;

    let scripts_dir = base_dir.join("Scripts");
    let autoexec_dir = base_dir.join("AutoExec");

    // Create directories if they don't exist
    if !scripts_dir.exists() {
        fs::create_dir_all(&scripts_dir)
            .map_err(|e| format!("Failed to create Scripts directory: {}", e))?;
    }

    if !autoexec_dir.exists() {
        fs::create_dir_all(&autoexec_dir)
            .map_err(|e| format!("Failed to create AutoExec directory: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
fn read_file_tree(app: AppHandle) -> Result<Vec<FileNode>, String> {
    let base_dir = get_base_directory(&app)?;

    let mut nodes = Vec::new();

    // Read Scripts directory
    let scripts_dir = base_dir.join("Scripts");
    if scripts_dir.exists() {
        let scripts_node = read_directory(&scripts_dir, &base_dir, "scripts", "Scripts")?;
        nodes.push(scripts_node);
    }

    // Read AutoExec directory
    let autoexec_dir = base_dir.join("AutoExec");
    if autoexec_dir.exists() {
        let autoexec_node = read_directory(&autoexec_dir, &base_dir, "autoexec", "AutoExec")?;
        nodes.push(autoexec_node);
    }

    Ok(nodes)
}

#[tauri::command]
fn read_file_content(app: AppHandle, relative_path: String) -> Result<String, String> {
    let base_dir = get_base_directory(&app)?;
    let file_path = base_dir.join(relative_path);

    // Security check: ensure the file is within the base directory
    let canonical_base = base_dir
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize base directory: {}", e))?;
    let canonical_file = file_path
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize file path: {}", e))?;

    if !canonical_file.starts_with(canonical_base) {
        return Err("Access denied: file is outside allowed directory".to_string());
    }
    fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
fn save_file(
    app: AppHandle,
    filename: String,
    folder: String,
    content: String,
) -> Result<String, String> {
    let base_dir = get_base_directory(&app)?;

    // Validate folder is either "Scripts" or "AutoExec"
    if folder != "Scripts" && folder != "AutoExec" {
        return Err("Invalid folder: must be 'Scripts' or 'AutoExec'".to_string());
    }

    let folder_path = base_dir.join(&folder);

    // Ensure the folder exists
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path)
            .map_err(|e| format!("Failed to create {} directory: {}", folder, e))?;
    }

    // Sanitize filename to prevent path traversal
    let safe_filename = filename
        .replace('/', "")
        .replace('\\', "")
        .replace("..", "");

    if safe_filename.is_empty() {
        return Err("Invalid filename".to_string());
    }

    let file_path = folder_path.join(&safe_filename);

    // Write the file
    fs::write(&file_path, content).map_err(|e| format!("Failed to write file: {}", e))?;

    // Return the relative path
    let relative_path = file_path
        .strip_prefix(&base_dir)
        .map_err(|e| format!("Failed to get relative path: {}", e))?
        .to_string_lossy()
        .to_string();

    Ok(relative_path)
}

fn read_directory(path: &Path, base_dir: &Path, id: &str, name: &str) -> Result<FileNode, String> {
    let mut children = Vec::new();
    let mut id_counter = 0;

    let entries = fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory {}: {}", path.display(), e))?;

    let mut sorted_entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    sorted_entries.sort_by_key(|e| e.path());

    for entry in sorted_entries {
        let entry_path = entry.path();
        let entry_name = entry.file_name().to_string_lossy().to_string();

        let entry_id = format!("{}-{}", id, id_counter);
        id_counter += 1;

        if entry_path.is_dir() {
            let child_node = read_directory(&entry_path, base_dir, &entry_id, &entry_name)?;
            children.push(child_node);
        } else {
            // Calculate relative path from base directory
            let relative_path = entry_path
                .strip_prefix(base_dir)
                .map_err(|e| format!("Failed to get relative path: {}", e))?
                .to_string_lossy()
                .to_string();

            children.push(FileNode::File {
                id: entry_id,
                name: entry_name,
                path: relative_path,
            });
        }
    }

    Ok(FileNode::Folder {
        id: id.to_string(),
        name: name.to_string(),
        children,
    })
}

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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_scripts_path,
            initialize_directories,
            read_file_tree,
            read_file_content,
            save_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
