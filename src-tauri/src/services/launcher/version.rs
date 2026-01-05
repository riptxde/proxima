use super::ipc;
use super::paths::LauncherPaths;

/// Fetch the latest Roblox version for the given channel
pub fn get_latest_version(channel: &str) -> Result<String, String> {
    println!("[*] Fetching latest Roblox version...");
    let _ = ipc::send_progress(5, "Fetching latest Roblox version...");

    // Build the version fetch URL based on channel
    // Empty or "LIVE" = production channel
    // Other channels use /channel/{channel} path
    let url = if channel.is_empty() || channel.eq_ignore_ascii_case("LIVE") {
        "https://clientsettings.roblox.com/v2/client-version/WindowsPlayer".to_string()
    } else {
        format!(
            "https://clientsettings.roblox.com/v2/client-version/WindowsPlayer/channel/{}",
            channel
        )
    };

    println!("[*] Fetching from: {}", url);

    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to fetch version: {}", e))?;

    let data: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed to parse version response: {}", e))?;

    let version = data
        .get("clientVersionUpload")
        .and_then(|v| v.as_str())
        .ok_or("Version field not found in response")?
        .to_string();

    println!("[*] Latest version: {}", version);
    let _ = ipc::send_progress(10, &format!("Found version: {}", version));
    Ok(version)
}

/// Check if a Roblox version is already installed
pub fn is_installed(paths: &LauncherPaths, version: &str) -> bool {
    let version_dir = paths.versions_dir.join(version);
    let roblox_exe = version_dir.join("RobloxPlayerBeta.exe");
    roblox_exe.exists()
}

/// Get the version directory path
pub fn get_version_dir(paths: &LauncherPaths, version: &str) -> std::path::PathBuf {
    paths.versions_dir.join(version)
}

/// Build the channel path for setup CDN
/// Empty or "LIVE" = root path
/// Other channels use /channel/{channel}/ path
pub fn build_channel_path(channel: &str) -> String {
    if channel.is_empty() || channel.eq_ignore_ascii_case("LIVE") {
        String::new()
    } else {
        format!("channel/{}/", channel)
    }
}
