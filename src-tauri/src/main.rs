// Prevents additional console window on Windows
// Comment out the line below to see force consoles to display launcher output
#![cfg_attr(all(windows), windows_subsystem = "windows")]

use std::env;

fn main() {
    // Check for launcher mode BEFORE initializing Tauri
    let args: Vec<String> = env::args().collect();

    // If --launch flag is present, run in launcher mode
    if args.contains(&"--launch".to_string()) {
        proxima_lib::launcher::run_launcher(&args);
        return;
    }

    // Otherwise, run the normal Tauri application
    proxima_lib::run();
}
