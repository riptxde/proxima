use std::ptr::null_mut;

#[tauri::command]
pub async fn launcher_launch() -> Result<(), String> {
    // Launch with empty URI to just start Roblox
    launch_with_uri("roblox-player:").await
}

async fn launch_with_uri(uri: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use winapi::um::shellapi::ShellExecuteW;
        use winapi::um::winuser::SW_SHOWNORMAL;

        unsafe {
            // Convert URI to wide string
            let operation: Vec<u16> = OsStr::new("open")
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();
            let file: Vec<u16> = OsStr::new(uri)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let result = ShellExecuteW(
                null_mut(),
                operation.as_ptr(),
                file.as_ptr(),
                null_mut(),
                null_mut(),
                SW_SHOWNORMAL,
            );

            // ShellExecuteW returns a value > 32 on success
            if result as usize <= 32 {
                return Err(format!("Failed to launch Roblox: error code {}", result as usize));
            }
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Launcher is only supported on Windows".to_string())
    }
}
