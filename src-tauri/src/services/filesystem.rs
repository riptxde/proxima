use crate::models::FileNode;
use std::fs;
use std::path::Path;
use tauri::AppHandle;

use super::paths;

/// Initialize the Scripts and AutoExec directories if they don't exist
pub fn initialize_directories(app: &AppHandle) -> Result<(), String> {
    let base_dir = paths::get_base_directory(app)?;

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

/// Build the file tree for Scripts and AutoExec directories
pub fn build_file_tree(app: &AppHandle) -> Result<Vec<FileNode>, String> {
    let base_dir = paths::get_base_directory(app)?;
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

/// Recursively read a directory and build a FileNode tree
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
