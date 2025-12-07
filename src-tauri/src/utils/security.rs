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

/// Validate that a relative folder path is within Scripts or AutoExec directories
/// This checks the path before it exists, using string validation
pub fn validate_scripts_folder(folder: &str) -> Result<(), String> {
    // Normalize path separators
    let normalized = folder.replace('\\', "/");

    // Check if it starts with scripts/ or autoexec/ or is exactly scripts or autoexec
    let is_valid = normalized.starts_with("scripts/")
        || normalized.starts_with("autoexec/")
        || normalized == "scripts"
        || normalized == "autoexec";

    if !is_valid {
        return Err("Invalid folder: must be within 'Scripts' or 'AutoExec' directory".to_string());
    }

    // Ensure no path traversal attempts
    if normalized.contains("..") {
        return Err("Invalid folder: path traversal is not allowed".to_string());
    }

    Ok(())
}

/// Validate that a file path is within Scripts or AutoExec after it has been created
pub fn validate_scripts_path(file_path: &Path, base_dir: &Path) -> Result<(), String> {
    let scripts_path = base_dir.join("scripts");
    let autoexec_path = base_dir.join("autoexec");

    let is_in_scripts = validate_path(file_path, &scripts_path).is_ok();
    let is_in_autoexec = validate_path(file_path, &autoexec_path).is_ok();

    if !is_in_scripts && !is_in_autoexec {
        return Err(
            "Access denied: file path is outside Scripts and AutoExec directories".to_string(),
        );
    }

    Ok(())
}

/// Sanitize a filename to prevent path traversal attacks
pub fn sanitize_filename(filename: &str) -> Result<String, String> {
    let safe = filename.replace(['/', '\\'], "").replace("..", "");

    if safe.is_empty() {
        return Err("Invalid filename".to_string());
    }

    Ok(safe)
}
