use serde::{Deserialize, Serialize};
use std::fs;

use super::paths::LauncherPaths;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherSettings {
    pub channel: String,
    pub version_override: String,
    #[serde(default = "default_cooldown")]
    pub cooldown: u64,
    #[serde(default)]
    pub multi_instance: bool,
}

fn default_cooldown() -> u64 {
    60
}

impl Default for LauncherSettings {
    fn default() -> Self {
        Self {
            channel: String::new(),
            version_override: String::new(),
            cooldown: default_cooldown(),
            multi_instance: false,
        }
    }
}

impl LauncherSettings {
    /// Load launcher settings from the settings file
    pub fn load(paths: &LauncherPaths) -> Self {
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

        // Return default settings if file doesn't exist or parsing fails
        Self::default()
    }
}
