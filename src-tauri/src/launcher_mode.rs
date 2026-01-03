// Proxima Roblox Launcher Mode
// Launcher functionality that runs when the executable is invoked with launcher arguments

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::services::launcher_ipc;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LauncherSettings {
    channel: String,
    version_override: String,
}

struct LauncherPaths {
    settings_file: PathBuf,
    versions_dir: PathBuf,
}

impl LauncherPaths {
    fn new() -> Self {
        let base_dir = Self::get_base_dir();
        let settings_file = base_dir.join("settings.json");
        let versions_dir = base_dir.join("roblox_versions");

        Self {
            settings_file,
            versions_dir,
        }
    }

    fn get_base_dir() -> PathBuf {
        #[cfg(debug_assertions)]
        {
            // Development: use @dev directory
            // Exe is at: src-tauri/target/debug/proxima.exe
            // We need to go up to project root, then into @dev
            let exe_path = env::current_exe().expect("Failed to get executable path");
            let workspace_root = exe_path
                .parent() // src-tauri/target/debug
                .and_then(|p| p.parent()) // src-tauri/target
                .and_then(|p| p.parent()) // src-tauri
                .and_then(|p| p.parent()) // project root
                .expect("Failed to get workspace root");
            workspace_root.join("@dev")
        }

        #[cfg(not(debug_assertions))]
        {
            // Production: use executable directory
            let exe_path = env::current_exe().expect("Failed to get executable path");
            exe_path
                .parent()
                .expect("Failed to get executable directory")
                .to_path_buf()
        }
    }
}

fn load_settings(paths: &LauncherPaths) -> LauncherSettings {
    if paths.settings_file.exists() {
        if let Ok(content) = fs::read_to_string(&paths.settings_file) {
            if let Ok(root) = serde_json::from_str::<serde_json::Value>(&content) {
                // Extract launcher settings from the main settings file
                // Structure: { "settings": { "launcher": { ... } } }
                if let Some(settings) = root.get("settings") {
                    if let Some(launcher) = settings.get("launcher") {
                        if let Ok(launcher_settings) = serde_json::from_value(launcher.clone()) {
                            return launcher_settings;
                        }
                    }
                }
            }
        }
    }

    // Default settings
    LauncherSettings {
        channel: String::new(),
        version_override: String::new(),
    }
}

fn get_latest_version(channel: &str) -> Result<String, String> {
    println!("[*] Fetching latest Roblox version...");
    let _ = launcher_ipc::send_progress(5, "Fetching latest Roblox version...");

    // Build the version fetch URL based on channel
    // Empty or "LIVE" = production channel
    // Other channels use /channel/{channel} path
    let url = if channel.is_empty() || channel.eq_ignore_ascii_case("LIVE") {
        "https://clientsettings.roblox.com/v2/client-version/WindowsPlayer".to_string()
    } else {
        format!("https://clientsettings.roblox.com/v2/client-version/WindowsPlayer/channel/{}", channel)
    };

    println!("[*] Fetching from: {}", url);

    let response = reqwest::blocking::get(&url).map_err(|e| format!("Failed to fetch version: {}", e))?;

    let data: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed to parse version response: {}", e))?;

    let version = data
        .get("clientVersionUpload")
        .and_then(|v| v.as_str())
        .ok_or("Version field not found in response")?
        .to_string();

    println!("[*] Latest version: {}", version);
    let _ = launcher_ipc::send_progress(10, &format!("Found version: {}", version));
    Ok(version)
}

fn is_installed(paths: &LauncherPaths, version: &str) -> bool {
    let version_dir = paths.versions_dir.join(version);
    let roblox_exe = version_dir.join("RobloxPlayerBeta.exe");
    roblox_exe.exists()
}

fn download_and_install(paths: &LauncherPaths, version: &str, channel: &str) -> Result<(), String> {
    println!("[*] Downloading and installing Roblox version: {}", version);
    let _ = launcher_ipc::send_progress(15, "Preparing download...");

    let version_dir = paths.versions_dir.join(version);

    // Build channel path for setup CDN
    // Empty or "LIVE" = root path
    // Other channels use /channel/{channel}/ path
    let channel_path = if channel.is_empty() || channel.eq_ignore_ascii_case("LIVE") {
        String::new()
    } else {
        format!("channel/{}/", channel)
    };

    // Fetch manifest
    let manifest_url = format!(
        "https://setup.rbxcdn.com/{}{}-rbxPkgManifest.txt",
        channel_path, version
    );

    println!("[*] Fetching manifest from: {}", manifest_url);
    let _ = launcher_ipc::send_progress(20, "Fetching manifest...");

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
        let error_msg = format!("Invalid version: No packages found in manifest. Version '{}' does not exist.", version);
        return Err(error_msg);
    }

    // Now that we know the version is valid, create the directory
    // Clean up if exists
    if version_dir.exists() {
        fs::remove_dir_all(&version_dir).map_err(|e| format!("Failed to clean version dir: {}", e))?;
    }

    fs::create_dir_all(&version_dir).map_err(|e| format!("Failed to create version dir: {}", e))?;

    let _ = launcher_ipc::send_progress(25, &format!("Downloading {} packages...", packages.len()));

    // Download and extract packages
    for (i, package) in packages.iter().enumerate() {
        println!("[*] [{}/{}] Downloading {}...", i + 1, packages.len(), package);

        // Progress from 25% to 85% based on package download
        let progress = 25 + ((i as f32 / packages.len() as f32) * 60.0) as u8;
        let _ = launcher_ipc::send_progress(progress, &format!("Downloading {} ({}/{})", package, i + 1, packages.len()));

        let package_url = format!("https://setup.rbxcdn.com/{}{}-{}", channel_path, version, package);

        let package_data = reqwest::blocking::get(&package_url)
            .map_err(|e| format!("Failed to download {}: {}", package, e))?
            .bytes()
            .map_err(|e| format!("Failed to read {}: {}", package, e))?;

        // Extract package
        extract_package(&version_dir, package, &package_data)?;
    }

    let _ = launcher_ipc::send_progress(85, "Creating configuration...");

    // Create AppSettings.xml
    let app_settings = r#"<?xml version="1.0" encoding="UTF-8"?>
<Settings>
    <ContentFolder>content</ContentFolder>
    <BaseUrl>http://www.roblox.com</BaseUrl>
</Settings>"#;

    fs::write(version_dir.join("AppSettings.xml"), app_settings)
        .map_err(|e| format!("Failed to create AppSettings.xml: {}", e))?;

    println!("[*] Installation complete!");
    let _ = launcher_ipc::send_progress(90, "Installation complete");
    Ok(())
}

fn extract_package(version_dir: &Path, package_name: &str, data: &[u8]) -> Result<(), String> {
    use std::io::Cursor;
    use zip::ZipArchive;

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
    let mut archive = ZipArchive::new(cursor).map_err(|e| format!("Failed to open zip: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to read zip entry: {}", e))?;

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

fn launch_roblox(paths: &LauncherPaths, version: &str, uri: Option<&str>) -> Result<(), String> {
    let roblox_exe = paths.versions_dir.join(version).join("RobloxPlayerBeta.exe");

    if !roblox_exe.exists() {
        return Err(format!("Roblox not found at: {}", roblox_exe.display()));
    }

    println!("[*] Launching Roblox from: {}", roblox_exe.display());
    let _ = launcher_ipc::send_progress(95, "Launching Roblox...");

    let mut cmd = Command::new(&roblox_exe);

    if let Some(uri) = uri {
        cmd.arg(uri);
        println!("[*] With URI: {}", uri);
    }

    cmd.current_dir(roblox_exe.parent().unwrap())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    cmd.spawn()
        .map_err(|e| format!("Failed to launch Roblox: {}", e))?;

    println!("[*] Roblox launched successfully!");
    let _ = launcher_ipc::send_progress(100, "Roblox launched successfully!");
    Ok(())
}

pub fn run_launcher(args: &[String]) {
    println!("========================================");
    println!("Proxima Roblox Launcher");
    println!("========================================");

    let paths = LauncherPaths::new();
    let settings = load_settings(&paths);

    println!("[*] Settings loaded:");
    println!("    - Channel: {}", if settings.channel.is_empty() { "live" } else { &settings.channel });
    println!("    - Version Override: {}", if settings.version_override.is_empty() { "None" } else { &settings.version_override });

    let _ = launcher_ipc::send_progress(0, "Initializing launcher...");

    // Parse launch URI from arguments
    // Args structure: [exe_path, "--launch", "roblox-player://..."]
    let launch_uri = args.iter()
        .position(|arg| arg == "--launch")
        .and_then(|idx| args.get(idx + 1))
        .filter(|arg| !arg.starts_with("--"))
        .map(|s| s.as_str());

    if let Some(uri) = launch_uri {
        println!("[*] Received launch URI: {}", uri);
    }

    // Determine version to use
    let version = if !settings.version_override.is_empty() {
        println!("[*] Using version override: {}", settings.version_override);
        settings.version_override.clone()
    } else {
        match get_latest_version(&settings.channel) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[!] Failed to get version: {}", e);
                let _ = launcher_ipc::send_progress_with_error(0, "Failed to get version", Some(e.clone()));
                std::process::exit(1);
            }
        }
    };

    // Create versions directory
    if let Err(e) = fs::create_dir_all(&paths.versions_dir) {
        eprintln!("[!] Failed to create Versions directory: {}", e);
        std::process::exit(1);
    }

    // Check if version is installed
    if !is_installed(&paths, &version) {
        println!("[*] Version {} not installed, downloading...", version);
        if let Err(e) = download_and_install(&paths, &version, &settings.channel) {
            eprintln!("[!] Failed to install: {}", e);
            let _ = launcher_ipc::send_progress_with_error(0, "Installation failed", Some(e.clone()));
            std::process::exit(1);
        }
    } else {
        println!("[*] Version {} is already installed", version);
    }

    // Launch Roblox
    if let Err(e) = launch_roblox(&paths, &version, launch_uri) {
        eprintln!("[!] Failed to launch: {}", e);
        let _ = launcher_ipc::send_progress_with_error(0, "Launch failed", Some(e.clone()));
        std::process::exit(1);
    }

    println!("[*] Done!");
}
