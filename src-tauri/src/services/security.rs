use std::path::Path;

/// Validate that a file path is within the allowed base directory
pub fn validate_path(file_path: &Path, base_dir: &Path) -> Result<(), String> {
    let canonical_base = base_dir
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize base directory: {}", e))?;
    let canonical_file = file_path
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize file path: {}", e))?;

    if !canonical_file.starts_with(canonical_base) {
        return Err("Access denied: file is outside allowed directory".to_string());
    }

    Ok(())
}

/// Sanitize a filename to prevent path traversal attacks
pub fn sanitize_filename(filename: &str) -> Result<String, String> {
    let safe = filename
        .replace('/', "")
        .replace('\\', "")
        .replace("..", "");

    if safe.is_empty() {
        return Err("Invalid filename".to_string());
    }

    Ok(safe)
}
