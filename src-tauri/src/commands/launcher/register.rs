use std::env;

#[tauri::command]
pub fn launcher_register(_app: tauri::AppHandle) -> Result<(), String> {
    register_protocol()
}

fn register_protocol() -> Result<(), String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let exe_path = env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?;

    let exe_path_str = exe_path
        .to_str()
        .ok_or("Executable path contains invalid characters")?;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu
        .create_subkey("Software\\Classes\\roblox-player")
        .map_err(|e| format!("Failed to create registry key: {}", e))?;

    key.set_value("", &"URL:roblox-player Protocol")
        .map_err(|e| format!("Failed to set default value: {}", e))?;

    key.set_value("URL Protocol", &"")
        .map_err(|e| format!("Failed to set URL Protocol: {}", e))?;

    // DefaultIcon
    let (icon_key, _) = hkcu
        .create_subkey("Software\\Classes\\roblox-player\\DefaultIcon")
        .map_err(|e| format!("Failed to create DefaultIcon key: {}", e))?;

    icon_key
        .set_value("", &exe_path_str)
        .map_err(|e| format!("Failed to set icon: {}", e))?;

    // Command - use --launch flag to invoke launcher mode
    let (cmd_key, _) = hkcu
        .create_subkey("Software\\Classes\\roblox-player\\shell\\open\\command")
        .map_err(|e| format!("Failed to create command key: {}", e))?;

    let command = format!(r#""{}" --launch "%1""#, exe_path_str);
    cmd_key
        .set_value("", &command)
        .map_err(|e| format!("Failed to set command: {}", e))?;

    Ok(())
}
