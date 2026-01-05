use std::fs;
use std::io::Cursor;
use std::path::Path;
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

    fs::create_dir_all(&version_dir)
        .map_err(|e| format!("Failed to create version dir: {}", e))?;

    let _ = ipc::send_progress(25, &format!("Downloading {} packages...", packages.len()));

    // Download and extract packages
    for (i, package) in packages.iter().enumerate() {
        println!("[*] [{}/{}] Downloading {}...", i + 1, packages.len(), package);

        // Progress from 25% to 85% based on package download
        let progress = 25 + ((i as f32 / packages.len() as f32) * 60.0) as u8;
        let _ = ipc::send_progress(
            progress,
            &format!("Downloading {} ({}/{})", package, i + 1, packages.len()),
        );

        let package_url = format!("https://setup.rbxcdn.com/{}{}-{}", channel_path, version, package);

        let package_data = reqwest::blocking::get(&package_url)
            .map_err(|e| format!("Failed to download {}: {}", package, e))?
            .bytes()
            .map_err(|e| format!("Failed to read {}: {}", package, e))?;

        // Extract package
        extract_package(&version_dir, package, &package_data)?;
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
        "content-platform-dictionaries.zip" => "PlatformContent/pc/shared_compression_dictionaries/",
        "extracontent-luapackages.zip" => "ExtraContent/LuaPackages/",
        "extracontent-translations.zip" => "ExtraContent/translations/",
        "extracontent-models.zip" => "ExtraContent/models/",
        "extracontent-textures.zip" => "ExtraContent/textures/",
        "extracontent-places.zip" => "ExtraContent/places/",
        _ => "",
    };

    let cursor = Cursor::new(data);
    let mut archive =
        ZipArchive::new(cursor).map_err(|e| format!("Failed to open zip: {}", e))?;

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
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // Extract file
        let mut output = fs::File::create(&target_path)
            .map_err(|e| format!("Failed to create file {}: {}", target_path.display(), e))?;

        std::io::copy(&mut file, &mut output)
            .map_err(|e| format!("Failed to extract file: {}", e))?;
    }

    Ok(())
}
