// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// Note: This is disabled when running in launcher mode to show console output
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

fn main() {
    // Check for launcher mode BEFORE initializing Tauri
    let args: Vec<String> = env::args().collect();

    // If --launch flag is present, run in launcher mode
    if args.contains(&"--launch".to_string()) {
        // Re-enable console in launcher mode for output visibility
        #[cfg(not(debug_assertions))]
        {
            use winapi::um::wincon::{AttachConsole, ATTACH_PARENT_PROCESS};
            unsafe {
                AttachConsole(ATTACH_PARENT_PROCESS);
            }
        }

        proxima_lib::launcher::run_launcher(&args);
        return;
    }

    // Otherwise, run the normal Tauri application
    proxima_lib::run();
}
