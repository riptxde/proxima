use crate::services::paths;
use std::fs;
use std::path::Path;
use tauri::AppHandle;

/// Get all AutoExec scripts with their contents
pub fn get_autoexec_scripts(app: &AppHandle) -> Vec<String> {
    let base_dir = match paths::get_base_directory(app) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get base directory for AutoExec: {}", e);
            return vec![];
        }
    };

    let autoexec_dir = base_dir.join("AutoExec");

    if !autoexec_dir.exists() {
        return vec![];
    }

    let mut scripts = Vec::new();
    collect_scripts(&autoexec_dir, &mut scripts);

    // Sort for consistent order
    scripts.sort_by(|a, b| a.0.cmp(&b.0));

    // Return just the contents
    scripts.into_iter().map(|(_, content)| content).collect()
}

/// Recursively collect script files and their contents
fn collect_scripts(dir: &Path, scripts: &mut Vec<(String, String)>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read directory {}: {}", dir.display(), e);
            return;
        }
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_dir() {
            // Recursively collect from subdirectories
            collect_scripts(&path, scripts);
        } else if path.is_file() {
            // Check if file has a valid extension
            if let Some(extension) = path.extension() {
                let ext = extension.to_string_lossy().to_lowercase();
                if ext == "lua" || ext == "luau" || ext == "txt" {
                    // Read the file content
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            let path_str = path.to_string_lossy().to_string();
                            scripts.push((path_str, content));
                        }
                        Err(e) => {
                            eprintln!("Failed to read file {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
    }
}
