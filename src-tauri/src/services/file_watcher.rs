use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

use super::paths;

const DEBOUNCE_DURATION_MS: u64 = 500;

/// Start watching Scripts and AutoExec directories for file changes
pub fn start_file_watcher(app: AppHandle) -> Result<(), String> {
    let base_dir = paths::get_base_directory(&app)?;
    let scripts_dir = base_dir.join("Scripts");
    let autoexec_dir = base_dir.join("AutoExec");

    log::info!("File watcher initialized");

    let app_clone = app.clone();
    std::thread::spawn(move || {
        if let Err(e) = watch_directories(app_clone, scripts_dir, autoexec_dir) {
            log::error!("File watcher error: {}", e);
        }
    });

    Ok(())
}

fn watch_directories(
    app: AppHandle,
    scripts_dir: PathBuf,
    autoexec_dir: PathBuf,
) -> Result<(), String> {
    let (tx, rx) = channel();

    let mut debouncer: Debouncer<RecommendedWatcher, FileIdMap> = new_debouncer(
        Duration::from_millis(DEBOUNCE_DURATION_MS),
        None,
        move |result: DebounceEventResult| {
            if let Ok(events) = result {
                if !events.is_empty() {
                    let _ = tx.send(());
                }
            }
        },
    )
    .map_err(|e| format!("Failed to create file watcher: {}", e))?;

    // Watch directories if they exist
    for (dir, name) in [(&scripts_dir, "Scripts"), (&autoexec_dir, "AutoExec")] {
        if dir.exists() {
            debouncer
                .watcher()
                .watch(dir, RecursiveMode::Recursive)
                .map_err(|e| format!("Failed to watch {} directory: {}", name, e))?;
        }
    }

    // Event loop - emit to frontend when files change
    while rx.recv().is_ok() {
        if let Err(e) = app.emit("file-tree-changed", ()) {
            log::error!("Failed to emit file tree change event: {}", e);
        }

        log::debug!("File watcher detected file tree change");
    }

    Ok(())
}
