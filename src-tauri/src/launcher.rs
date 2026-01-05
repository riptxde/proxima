// Proxima Roblox Launcher
// Launcher functionality that runs when the executable is invoked with launcher arguments

use std::fs;

use crate::services::launcher::{
    installation, ipc, mutex, paths::LauncherPaths, process, settings::LauncherSettings, version,
};

/// Main launcher entry point
///
/// Orchestrates the launcher flow:
/// 1. Connect to IPC WebSocket
/// 2. Acquire launcher mutex (queue if locked)
/// 3. Load settings
/// 4. Acquire Roblox singleton mutex if multi-instance enabled
/// 5. Determine version to use
/// 6. Install if needed
/// 7. Launch Roblox
/// 8. Apply cooldown
/// 9. Hold Roblox mutex while processes run (master only)
pub fn run_launcher(args: &[String]) {
    // Establish persistent WebSocket connection for progress updates
    let _ = ipc::connect();

    // Try to acquire the launcher mutex
    #[cfg(windows)]
    let _mutex = match mutex::NamedMutex::try_acquire(mutex::LAUNCHER_MUTEX_NAME) {
        Ok(mutex) => {
            println!("[*] Launcher mutex acquired, proceeding...");
            mutex
        }
        Err(is_locked) => {
            if is_locked {
                println!("[*] Another launcher is running, waiting in queue...");

                // Join the queue
                let launcher_id = uuid::Uuid::new_v4().to_string();
                let _ = ipc::join_queue(launcher_id.clone());

                // Wait for the mutex to become available
                match mutex::NamedMutex::wait_and_acquire(mutex::LAUNCHER_MUTEX_NAME) {
                    Ok(mutex) => {
                        println!("[*] Launcher mutex acquired after waiting");
                        mutex
                    }
                    Err(e) => {
                        eprintln!("[!] Failed to wait for launcher mutex: {}", e);
                        let _ = ipc::disconnect();
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("[!] Failed to create launcher mutex");
                let _ = ipc::disconnect();
                std::process::exit(1);
            }
        }
    };

    let paths = LauncherPaths::new();
    let settings = LauncherSettings::load(&paths);

    println!("[*] Settings loaded:");
    println!(
        "    - Channel: {}",
        if settings.channel.is_empty() {
            "live"
        } else {
            &settings.channel
        }
    );
    println!(
        "    - Version Override: {}",
        if settings.version_override.is_empty() {
            "None"
        } else {
            &settings.version_override
        }
    );
    println!("    - Multi-Instance: {}", settings.multi_instance);
    println!("    - Cooldown: {} seconds", settings.cooldown);

    let _ = ipc::send_progress(0, "Initializing launcher...");

    // Try to acquire Roblox singleton mutex if multi-instance is enabled
    #[cfg(windows)]
    let roblox_singleton = if settings.multi_instance {
        match mutex::NamedMutex::try_acquire_roblox_singleton() {
            Some(mutex) => {
                println!("[*] Multi-instance enabled - Instance type: Master");
                let _ = ipc::send_progress(0, "Initializing launcher (Master)...");
                Some(mutex)
            }
            None => {
                println!("[*] Multi-instance enabled - Instance type: Slave");
                let _ = ipc::send_progress(0, "Initializing launcher (Slave)...");
                None
            }
        }
    } else {
        None
    };

    // Parse launch URI from arguments
    // Args structure: [exe_path, "--launch", "roblox-player://..."]
    let launch_uri = args
        .iter()
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
        match version::get_latest_version(&settings.channel) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[!] Failed to get version: {}", e);
                let _ = ipc::send_progress_with_error(0, "Failed to get version", Some(e.clone()));
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
    if !version::is_installed(&paths, &version) {
        println!("[*] Version {} not installed, downloading...", version);
        if let Err(e) = installation::download_and_install(&paths, &version, &settings.channel) {
            eprintln!("[!] Failed to install: {}", e);
            let _ = ipc::send_progress_with_error(0, "Installation failed", Some(e.clone()));
            std::process::exit(1);
        }
    } else {
        println!("[*] Version {} is already installed", version);
    }

    // Launch Roblox
    if let Err(e) = process::launch_roblox(&paths, &version, launch_uri) {
        eprintln!("[!] Failed to launch: {}", e);
        let _ = ipc::send_progress_with_error(0, "Launch failed", Some(e.clone()));
        std::process::exit(1);
    }

    // Close WebSocket connection to remove from queue immediately
    let _ = ipc::disconnect();

    // Apply cooldown before releasing launcher mutex
    if settings.cooldown > 0 {
        println!("[*] Applying cooldown of {} seconds...", settings.cooldown);
        std::thread::sleep(std::time::Duration::from_secs(settings.cooldown));
        println!("[*] Cooldown complete");
    }

    // Explicitly drop the launcher mutex to release it BEFORE holding Roblox mutex
    drop(_mutex);
    println!("[*] Launcher mutex released");

    // If we're the master instance, hold the Roblox singleton while processes are running
    #[cfg(windows)]
    if let Some(ref roblox_mutex) = roblox_singleton {
        roblox_mutex.hold_while_roblox_running();
    }

    println!("[*] Done!");
}
