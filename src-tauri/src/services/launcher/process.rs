use std::process::{Command, Stdio};
use sysinfo::System;

use super::ipc;
use super::paths::LauncherPaths;
use super::version;

/// Launch Roblox with an optional URI
pub fn launch_roblox(
    paths: &LauncherPaths,
    version: &str,
    uri: Option<&str>,
) -> Result<(), String> {
    let roblox_exe = version::get_version_dir(paths, version).join("RobloxPlayerBeta.exe");

    if !roblox_exe.exists() {
        return Err(format!("Roblox not found at: {}", roblox_exe.display()));
    }

    println!("[*] Launching Roblox from: {}", roblox_exe.display());
    let _ = ipc::send_progress(95, "Launching Roblox...");

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
    let _ = ipc::send_progress(100, "Roblox launched successfully!");
    Ok(())
}

/// Kill all running Roblox processes (Master only)
#[cfg(windows)]
pub fn kill_all_roblox_processes() {
    println!("[*] Killing all existing Roblox processes...");

    let process_names = [
        "RobloxPlayerBeta.exe",
        "Roblox.exe",
        "RobloxCrashHandler.exe",
    ];

    for process_name in &process_names {
        let _ = Command::new("taskkill")
            .args(&["/F", "/IM", process_name])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

/// Check if any RobloxPlayerBeta.exe processes are running
pub fn has_roblox_processes() -> bool {
    let mut system = System::new_all();
    system.refresh_all();

    system.processes().values().any(|process| {
        process
            .name()
            .to_string_lossy()
            .eq_ignore_ascii_case("RobloxPlayerBeta.exe")
    })
}
