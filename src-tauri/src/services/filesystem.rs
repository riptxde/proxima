use crate::models::FileNode;
use crate::utils::ignore::{ensure_ignore_file, IgnorePatterns};
use std::fs;
use std::path::Path;
use tauri::AppHandle;

use crate::utils::paths;

/// Initialize the scripts and autoexec directories if they don't exist
pub fn initialize_directories(app: &AppHandle) -> Result<(), String> {
    let base_dir = paths::get_base_directory(app)?;

    let scripts_dir = base_dir.join("scripts");
    let autoexec_dir = base_dir.join("autoexec");

    let mut created_dirs = Vec::new();

    // Create directories if they don't exist
    if !scripts_dir.exists() {
        fs::create_dir_all(&scripts_dir)
            .map_err(|e| format!("Failed to create Scripts directory: {}", e))?;
        created_dirs.push("Scripts");
    }

    if !autoexec_dir.exists() {
        fs::create_dir_all(&autoexec_dir)
            .map_err(|e| format!("Failed to create AutoExec directory: {}", e))?;
        created_dirs.push("AutoExec");
    }

    // Create default .proximaignore files
    ensure_ignore_file(&scripts_dir)?;
    ensure_ignore_file(&autoexec_dir)?;

    // Log directory initialization
    if !created_dirs.is_empty() {
        log::info!("Initialized directories: {}", created_dirs.join(", "));
    }

    Ok(())
}

/// Build the file tree for scripts and autoexec directories
pub fn build_file_tree(app: &AppHandle) -> Result<Vec<FileNode>, String> {
    let base_dir = paths::get_base_directory(app)?;
    let mut nodes = Vec::new();

    // Read scripts directory
    let scripts_dir = base_dir.join("scripts");
    if scripts_dir.exists() {
        let ignore_file = scripts_dir.join(".proximaignore");
        let ignore_patterns = IgnorePatterns::from_file(&ignore_file)?;
        let scripts_node = read_directory(
            &scripts_dir,
            &base_dir,
            "scripts",
            "Scripts",
            &ignore_patterns,
        )?;
        nodes.push(scripts_node);
    }

    // Read autoexec directory
    let autoexec_dir = base_dir.join("autoexec");
    if autoexec_dir.exists() {
        let ignore_file = autoexec_dir.join(".proximaignore");
        let ignore_patterns = IgnorePatterns::from_file(&ignore_file)?;
        let autoexec_node = read_directory(
            &autoexec_dir,
            &base_dir,
            "autoexec",
            "AutoExec",
            &ignore_patterns,
        )?;
        nodes.push(autoexec_node);
    }

    Ok(nodes)
}

/// Recursively read a directory and build a FileNode tree
fn read_directory(
    path: &Path,
    base_dir: &Path,
    id: &str,
    name: &str,
    ignore_patterns: &IgnorePatterns,
) -> Result<FileNode, String> {
    let mut children = Vec::new();

    let entries = fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory {}: {}", path.display(), e))?;

    let mut sorted_entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    // Sort: folders first (alphabetically), then files (alphabetically)
    sorted_entries.sort_by(|a, b| {
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less, // a is folder, b is file -> a comes first
            (false, true) => std::cmp::Ordering::Greater, // a is file, b is folder -> b comes first
            _ => a.file_name().cmp(&b.file_name()),    // both same type -> alphabetical by name
        }
    });

    let mut id_counter = 0;
    for entry in sorted_entries.into_iter() {
        let entry_path = entry.path();
        let entry_name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry_path.is_dir();

        // Calculate relative path from the parent directory being scanned
        let relative_from_parent = Path::new(&entry_name);

        // Check if this entry should be ignored
        if ignore_patterns.is_ignored(relative_from_parent, is_dir) {
            continue;
        }

        let entry_id = format!("{}-{}", id, id_counter);
        id_counter += 1;

        // Calculate relative path from base directory with forward slashes
        let relative_path = entry_path
            .strip_prefix(base_dir)
            .map_err(|e| format!("Failed to get relative path: {}", e))?
            .to_string_lossy()
            .replace('\\', "/");

        if is_dir {
            let child_node = read_directory(
                &entry_path,
                base_dir,
                &entry_id,
                &entry_name,
                ignore_patterns,
            )?;
            children.push(child_node);
        } else {
            children.push(FileNode::File {
                id: entry_id,
                name: entry_name,
                path: relative_path,
            });
        }
    }

    // Calculate relative path for the current folder
    let folder_path = path
        .strip_prefix(base_dir)
        .map_err(|e| format!("Failed to get relative path: {}", e))?
        .to_string_lossy()
        .replace('\\', "/");

    Ok(FileNode::Folder {
        id: id.to_string(),
        name: name.to_string(),
        path: folder_path,
        children,
    })
}
