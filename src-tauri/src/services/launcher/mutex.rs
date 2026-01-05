#[cfg(windows)]
use std::ptr::null_mut;
#[cfg(windows)]
use winapi::shared::winerror::ERROR_ALREADY_EXISTS;
#[cfg(windows)]
use winapi::um::errhandlingapi::GetLastError;
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;
#[cfg(windows)]
use winapi::um::synchapi::{CreateMutexW, ReleaseMutex, WaitForSingleObject};
#[cfg(windows)]
use winapi::um::winbase::{INFINITE, WAIT_OBJECT_0};

use super::process;

/// RAII wrapper for Windows named mutex
#[cfg(windows)]
pub struct NamedMutex {
    handle: winapi::shared::ntdef::HANDLE,
    is_master: bool,
}

#[cfg(windows)]
impl NamedMutex {
    /// Try to acquire the named mutex
    /// Returns Ok(NamedMutex) if acquired, Err if mutex is locked
    pub fn try_acquire(name: &str) -> Result<Self, bool> {
        unsafe {
            // Convert name to wide string
            let wide_name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

            // Create or open the named mutex
            let handle = CreateMutexW(null_mut(), 1, wide_name.as_ptr());

            if handle.is_null() {
                return Err(false);
            }

            // Check if mutex already existed
            let last_error = GetLastError();
            let is_master = last_error != ERROR_ALREADY_EXISTS;

            if !is_master {
                // Mutex exists, meaning another launcher is running
                CloseHandle(handle);
                return Err(true);
            }

            Ok(NamedMutex { handle, is_master })
        }
    }

    /// Wait for the mutex to become available, then acquire it
    pub fn wait_and_acquire(name: &str) -> Result<Self, String> {
        unsafe {
            // Convert name to wide string
            let wide_name: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();

            // Open the existing mutex
            let handle = CreateMutexW(null_mut(), 0, wide_name.as_ptr());

            if handle.is_null() {
                return Err("Failed to open mutex".to_string());
            }

            // Wait indefinitely for the mutex
            let wait_result = WaitForSingleObject(handle, INFINITE);

            if wait_result != WAIT_OBJECT_0 {
                CloseHandle(handle);
                return Err("Failed to wait for mutex".to_string());
            }

            Ok(NamedMutex {
                handle,
                is_master: false,
            })
        }
    }

    /// Try to acquire the Roblox singleton mutex (for multi-instance)
    /// Returns Some(mutex) if we became master, None if slave (mutex already held)
    pub fn try_acquire_roblox_singleton() -> Option<Self> {
        unsafe {
            const ROBLOX_MUTEX: &str = "ROBLOX_singletonMutex";
            let wide_name: Vec<u16> = ROBLOX_MUTEX
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

            // Try to create the mutex
            let handle = CreateMutexW(null_mut(), 1, wide_name.as_ptr());

            if handle.is_null() {
                return None;
            }

            // Check if we created it (master) or it already existed (slave)
            let last_error = GetLastError();

            if last_error == ERROR_ALREADY_EXISTS {
                // We're a slave - close handle and return None
                CloseHandle(handle);
                None
            } else {
                // We're the master - kill existing Roblox processes
                process::kill_all_roblox_processes();
                Some(NamedMutex {
                    handle,
                    is_master: true,
                })
            }
        }
    }

    /// Keep the mutex held while any RobloxPlayerBeta.exe processes are running
    /// Checks every 500ms and releases when all processes exit
    pub fn hold_while_roblox_running(&self) {
        if !self.is_master {
            return;
        }

        println!("[*] Holding Roblox singleton mutex while instances are running...");

        loop {
            // Check if any Roblox processes are still running
            if !process::has_roblox_processes() {
                println!("[*] No Roblox processes detected, releasing mutex");
                break;
            }

            // Sleep for 500ms before checking again
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}

#[cfg(windows)]
impl Drop for NamedMutex {
    fn drop(&mut self) {
        unsafe {
            ReleaseMutex(self.handle);
            CloseHandle(self.handle);
        }
    }
}

/// Mutex name constants
pub const LAUNCHER_MUTEX_NAME: &str = "Global\\ProximaRobloxLauncher";
