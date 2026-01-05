use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::sync::{Arc, Mutex};
use zip::ZipArchive;

use super::ipc;
use super::paths::LauncherPaths;
use super::version;

/// Download and install a Roblox version
pub fn download_and_install(
    paths: &LauncherPaths,
    version: &str,
    channel: &str,
) -> Result<(), String> {
    println!("[*] Downloading and installing Roblox version: {}", version);
    let _ = ipc::send_progress(15, "Preparing download...");

    let version_dir = version::get_version_dir(paths, version);
    let channel_path = version::build_channel_path(channel);

    // Fetch manifest
    let manifest_url = format!(
        "https://setup.rbxcdn.com/{}{}-rbxPkgManifest.txt",
        channel_path, version
    );

    println!("[*] Fetching manifest from: {}", manifest_url);
    let _ = ipc::send_progress(20, "Fetching manifest...");

    let manifest_text = reqwest::blocking::get(&manifest_url)
        .map_err(|e| format!("Failed to fetch manifest: {}", e))?
        .text()
        .map_err(|e| format!("Failed to read manifest: {}", e))?;

    // Parse manifest (v0 format, lines ending with .zip)
    let packages: Vec<&str> = manifest_text
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.ends_with(".zip"))
        .collect();

    println!("[*] Found {} packages in manifest", packages.len());

    // Validate manifest - if 0 packages, the version is invalid
    if packages.is_empty() {
        let error_msg = format!(
            "Invalid version: No packages found in manifest. Version '{}' does not exist.",
            version
        );
        return Err(error_msg);
    }

    // Now that we know the version is valid, create the directory
    // Clean up if exists
    if version_dir.exists() {
        fs::remove_dir_all(&version_dir)
            .map_err(|e| format!("Failed to clean version dir: {}", e))?;
    }

    fs::create_dir_all(&version_dir).map_err(|e| format!("Failed to create version dir: {}", e))?;

    let _ = ipc::send_progress(25, &format!("Downloading {} packages...", packages.len()));

    // Download and extract packages in parallel
    let version_dir = Arc::new(version_dir);
    let channel_path = Arc::new(channel_path);
    let total_packages = packages.len();
    let completed_count = Arc::new(Mutex::new(0usize));

    // Use scoped threads for parallel downloads and extractions
    let results: Vec<Result<(), String>> = std::thread::scope(|s| {
        packages
            .into_iter()
            .map(|package| {
                let version_dir = Arc::clone(&version_dir);
                let channel_path = Arc::clone(&channel_path);
                let version = version.to_string();
                let package = package.to_string();
                let completed_count = Arc::clone(&completed_count);

                s.spawn(move || {
                    println!("[*] Downloading {}...", package);

                    let package_url = format!(
                        "https://setup.rbxcdn.com/{}{}-{}",
                        channel_path, version, package
                    );

                    // Download with streaming to handle large files better
                    let mut response = reqwest::blocking::get(&package_url)
                        .map_err(|e| format!("Failed to download {}: {}", package, e))?;

                    // Read response body in chunks to avoid memory issues with large files
                    let mut package_data = Vec::new();
                    std::io::copy(&mut response, &mut package_data)
                        .map_err(|e| format!("Failed to read {}: {}", package, e))?;

                    let file_size = package_data.len();

                    // Extract package immediately after download
                    extract_package(&version_dir, &package, &package_data)?;

                    // Update progress
                    let mut count = completed_count.lock().unwrap();
                    *count += 1;
                    let current_count = *count;
                    let progress =
                        25 + ((current_count as f32 / total_packages as f32) * 60.0) as u8;
                    drop(count); // Release lock before sending progress

                    let _ = ipc::send_progress(
                        progress,
                        &format!("Extracted {} ({})", package, format_file_size(file_size)),
                    );

                    println!("[*] Completed {}", package);
                    Ok(())
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    });

    // Check for any errors
    for result in results {
        result?;
    }

    let _ = ipc::send_progress(85, "Creating configuration...");

    // Create AppSettings.xml
    let app_settings = r#"<?xml version="1.0" encoding="UTF-8"?>
<Settings>
    <ContentFolder>content</ContentFolder>
    <BaseUrl>http://www.roblox.com</BaseUrl>
</Settings>"#;

    fs::write(version_dir.join("AppSettings.xml"), app_settings)
        .map_err(|e| format!("Failed to create AppSettings.xml: {}", e))?;

    println!("[*] Installation complete!");
    let _ = ipc::send_progress(90, "Installation complete");
    Ok(())
}

/// Format file size in human-readable format
fn format_file_size(bytes: usize) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let bytes_f = bytes as f64;

    if bytes_f >= GB {
        format!("{:.2} GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}

/// Extract a Roblox package to the version directory
fn extract_package(version_dir: &Path, package_name: &str, data: &[u8]) -> Result<(), String> {
    // Package roots mapping
    let root = match package_name {
        "shaders.zip" => "shaders/",
        "ssl.zip" => "ssl/",
        "WebView2RuntimeInstaller.zip" => "WebView2RuntimeInstaller/",
        "content-avatar.zip" => "content/avatar/",
        "content-configs.zip" => "content/configs/",
        "content-fonts.zip" => "content/fonts/",
        "content-sky.zip" => "content/sky/",
        "content-sounds.zip" => "content/sounds/",
        "content-textures2.zip" => "content/textures/",
        "content-models.zip" => "content/models/",
        "content-textures3.zip" => "PlatformContent/pc/textures/",
        "content-terrain.zip" => "PlatformContent/pc/terrain/",
        "content-platform-fonts.zip" => "PlatformContent/pc/fonts/",
        "content-platform-dictionaries.zip" => {
            "PlatformContent/pc/shared_compression_dictionaries/"
        }
        "extracontent-luapackages.zip" => "ExtraContent/LuaPackages/",
        "extracontent-translations.zip" => "ExtraContent/translations/",
        "extracontent-models.zip" => "ExtraContent/models/",
        "extracontent-textures.zip" => "ExtraContent/textures/",
        "extracontent-places.zip" => "ExtraContent/places/",
        _ => "",
    };

    let cursor = Cursor::new(data);
    let mut archive = ZipArchive::new(cursor).map_err(|e| format!("Failed to open zip: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("Failed to read zip entry: {}", e))?;

        let file_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Skip directories
        if file.is_dir() {
            continue;
        }

        // Build target path with root prefix
        let target_path = version_dir.join(root).join(&file_path);

        // Create parent directories
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // Extract file
        let mut output = fs::File::create(&target_path)
            .map_err(|e| format!("Failed to create file {}: {}", target_path.display(), e))?;

        std::io::copy(&mut file, &mut output)
            .map_err(|e| format!("Failed to extract file: {}", e))?;
    }

    Ok(())
}
