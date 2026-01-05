use std::env;
use std::path::PathBuf;

/// Launcher-specific paths for settings and Roblox versions
pub struct LauncherPaths {
    pub settings_file: PathBuf,
    pub versions_dir: PathBuf,
}

impl LauncherPaths {
    pub fn new() -> Self {
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
